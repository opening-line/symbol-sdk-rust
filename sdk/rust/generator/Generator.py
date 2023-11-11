#!/usr/bin/python

from pathlib import Path
from enum import Enum
import catparser
import lark
from catparser.DisplayType import DisplayType

TRAITS = ("signature", "signer_public_key", "message")

class Generator:
    @staticmethod
    def generate(ast_models, output):
        print(f'python catbuffer generator called with output: {output}')
        generate_files(ast_models, Path(output))

def snake_to_camel(word):
    return ''.join(x.capitalize() or '_' for x in word.split('_'))

def get_type_of_trait(trait, ast_models):
    for ast_model in ast_models:
        if ast_model.display_type != DisplayType.STRUCT:
            continue
        for f in ast_model.fields:
            if f.name != trait:
                continue
            if type(f.field_type) == catparser.ast.Array:
                return f'Vec<{f.field_type.element_type.short_name.replace("uint", "u").replace("int", "i")}>'
            else:
                return f.field_type
            
def get_factory_type(ast_model):
    if hasattr(ast_model, 'factory_type'):
        return ast_model.factory_type
    return None

def get_factory_types(ast_models):
    factory_types = set([get_factory_type(ast_model) for ast_model in ast_models])
    factory_types.remove(None)
    return factory_types

def generate_files(ast_models, output_directory: Path):

    output_directory.mkdir(exist_ok=True)

    with open(output_directory / 'models.rs', 'w', encoding='utf8', newline='') as output_file:
        import copy
        output = ''
        output += 'use hex;'
        output += 'pub use crate::symbol::models_header::*;'

        for trait in TRAITS:
            type = get_type_of_trait(trait, ast_models)
            output += f'pub trait Trait{snake_to_camel(trait)} {{'
            output += f'fn get_{trait}(&self) -> &{type};'
            output += f'fn set_{trait}(&mut self, {trait}: {type});'
            output += '}'
            
        factory_types = get_factory_types(ast_models)

        for ast_model in ast_models:
            if ast_model.name in ('Signature', 'PublicKey'):
                continue
            
            ast_model = copy.deepcopy(ast_model)
            output += header_for_each_ast_model(ast_model)
            if ast_model.name in factory_types:
                factory = ast_model
                factory_name = factory.name
                products = []
                for ast_model in ast_models:
                    if get_factory_type(ast_model) == factory_name:
                        products.append(copy.deepcopy(ast_model))
                output += generate_factory(factory, products)
            elif ast_model.display_type == DisplayType.STRUCT:
                output += generate_struct(ast_model)
            elif ast_model.display_type == DisplayType.ENUM:
                output += generate_enum(ast_model)
            elif ast_model.display_type == DisplayType.BYTE_ARRAY:
                output += generate_bytearray(ast_model)
            elif ast_model.display_type == DisplayType.INTEGER:
                output += generate_integer(ast_model)
            else:
                raise 'Unexpected'

        output_file.write(output)
        
        
def generate_factory(factory, products):
    def update_field_type(field_type):
        if type(field_type) == lark.lexer.Token:
            pass
        elif type(field_type) == catparser.ast.FixedSizeInteger:
            field_type.short_name = field_type.short_name.replace("uint", "u").replace("int", "i")
        elif type(field_type) == catparser.ast.Array:
            element_type = field_type.element_type
            update_field_type(element_type)
        elif type(field_type) == str:
            pass
        else:
            raise "unexpected"
    
    def constantized_by(field_name, ast_model):
        for x in ast_model.initializers:
            if x.target_property_name == field_name:
                return x.value                    
        return None
            
    def is_size_of_other(field, ast_model):
        for other_field in ast_model.fields:
            if field.name == other_field.size:
                return other_field
        return None
    
    def skip_in_constructor(field):
        if field.name in ("signature"):
            return True
        return False
    
    def is_member(field, ast_model):
        if constantized_by(field.name, ast_model):
            return False
        if is_size_of_other(field, ast_model):
            return False
        if field.is_const:
            return False
        if field.is_reserved:
            return False
        if field.name == "size":
            return False
        return True
    
    def is_method(field):
        return constantized_by(field.name, factory)
    
    # prepare
    for f in factory.fields:
        update_field_type(f.field_type)
    for p in products:
        for f in p.fields:
            update_field_type(f.field_type)
            

    factory_name = factory.name
    ret = ''
    
    ret += f'pub enum {factory_name} {{'
    for p in products:
        ret += f'{p.name}({p.name}),'
    ret += '}'
    
    ret += f'impl {factory_name} {{'
    
    ret += 'pub fn size(&self) -> usize {'
    ret += 'match self {'
    for p in products:
        ret += f'Self::{p.name}(x) => x.size(),'
    ret += '}}'
    
    ret += 'pub fn deserialize(mut payload: &[u8]) -> Result<(Self, &[u8]), SymbolError> {'
    for f in factory.fields:
        if f.is_const:
            continue
        fn = f.name
        if constantized_by(fn, factory):
            if fn not in factory.discriminator:
                fn = '_' + fn
        
        ft = f.field_type
        fs = f.size
        if f.name == 'size':
            ret += f'if payload.len() < {fs} {{ return Err(SymbolError::SizeError{{expect: {fs}, real: payload.len()}}) }}'
        if type(ft) == catparser.ast.FixedSizeInteger:
            ret += f'let {fn} = {ft}::from_le_bytes(payload[..{fs}].try_into()?);'
            ret += f'payload = &payload[{fs}..];'
        elif type(ft) == catparser.ast.Array:
            ret += f'let mut {fn} = Vec::new();'
            ret += f'for _ in 0..{fs} {{'
            
            fte = ft.element_type
            if type(fte) == catparser.ast.FixedSizeInteger:
                ftes = fte.size
                # ften = fte.name
                ret += f'let mut bytes = [0u8; {ftes}];'
                ret += f'bytes.copy_from_slice(payload);'
                ret += f'let element = {fte}::from_le_bytes(bytes);'
                ret += f'payload = &payload[{ftes}..];'
                ret += f'{fn}.push(element);'
            elif type(fte) == str:
                ret += f'let element;'
                ret += f'(element, payload) = {fte}::deserialize(payload)?;'
                ret += f'{fn}.push(element);'
            else:
                raise "unexpected"
            ret += '}'
        else:
            ret += f'let {fn};'
            ret += f'({fn}, payload) = {ft}::deserialize(payload)?;'
        
        if f.name == 'size':
            ret += f'if size as usize >= payload.len() + {fs} {{ return Err(SymbolError::SizeError{{expect: size as usize, real: payload.len() + {fs} }}); }}'
        if f.is_reserved:
            ret += f'if {f.name} != 0 {{ return Err(SymbolError::ReservedIsNotZeroError({f.name} as u32)); }}'
        if constantized_by(f.name, factory):
            pass
        
        
    common_field_name_list = [f.name for f in factory.fields]
    ret += 'match ('
    for d in factory.discriminator:
        ret += f'{d}, '
    ret += ') {'
    for p in products:
        ret += '('
        for d in factory.discriminator:
            ret += f'{p.name}::{constantized_by(d, p)}, '
        ret += ') => {'
        for f in p.fields:
            if f.name in common_field_name_list:
                continue
            if f.is_const:
                continue
            fn = f.name
            if constantized_by(f.name, p):
                fn = '_' + fn
            
            ft = f.field_type
            fs = f.size
            if f.name == 'size':
                ret += f'if payload.len() < {fs} {{ return Err(SymbolError::SizeError{{expect: {fs}, real: payload.len()}}) }}'
            if type(ft) == catparser.ast.FixedSizeInteger:
                ret += f'let {fn} = {ft}::from_le_bytes(payload[..{fs}].try_into()?);'
                ret += f'payload = &payload[{fs}..];'
            elif type(ft) == catparser.ast.Array:
                ret += f'let mut {fn} = Vec::new();'
                ret += f'for _ in 0..{fs} {{'
                
                fte = ft.element_type
                if type(fte) == catparser.ast.FixedSizeInteger:
                    ftes = fte.size
                    # ften = fte.name
                    ret += f'let mut bytes = [0u8; {ftes}];'
                    ret += f'bytes.copy_from_slice(payload);'
                    ret += f'let element = {fte}::from_le_bytes(bytes);'
                    ret += f'payload = &payload[{ftes}..];'
                    ret += f'{fn}.push(element);'
                elif type(fte) == str:
                    ret += f'let element;'
                    ret += f'(element, payload) = {fte}::deserialize(payload)?;'
                    ret += f'{fn}.push(element);'
                else:
                    raise "unexpected"
                ret += '}'
            else:
                ret += f'let {fn};'
                ret += f'({fn}, payload) = {ft}::deserialize(payload)?;'
            
            if f.name == 'size':
                ret += f'if size as usize >= payload.len() + {fs} {{ return Err(SymbolError::SizeError{{expect: size as usize, real: payload.len() + {fs} }}); }}'
            if f.is_reserved:
                ret += f'if {f.name} != 0 {{ return Err(SymbolError::ReservedIsNotZeroError({f.name} as u32)); }}'
            if constantized_by(f.name, factory):
                pass
        ret += f'let self_ = {p.name} {{'
        for f in p.fields:
            if not is_member(f, p):
                continue
            ret += f'{f.name},'
        ret += '};'
            
        ret += f'Ok((Self::{p.name}(self_), payload))'    
        
        ret += '},'
    ret += '('
    for d in factory.discriminator:
        ret += f'other_{d}, '
    ret += ')'
    ret += ' => Err(SymbolError::MismatchError{ pattern: vec!['
    for d in factory.discriminator:
        ret += f'other_{d} as u32, '
    ret += '], '
    ret += f'place: "{factory_name}".to_string()'
    ret += '}),'
    ret += '}}' 
    
    ret += 'pub fn serialize(&self) -> Vec<u8> {'
    ret += 'match self {'
    for p in products:
        ret += f'Self::{p.name}(x) => x.serialize(),'
    ret += '}}'

    
    ret += '}'
    
    for p in products:
        ret += f'impl From<{p.name}> for {factory_name} {{ fn from(value: {p.name}) -> Self {{ Self::{p.name}(value) }} }}'
    
    return ret.replace('type', 'type_')
    
    
def generate_struct(ast_model):
    struct_name = ast_model.name
    
    def update_field_type(field_type):
        if type(field_type) == lark.lexer.Token:
            pass
        elif type(field_type) == catparser.ast.FixedSizeInteger:
            field_type.short_name = field_type.short_name.replace("uint", "u").replace("int", "i")
        elif type(field_type) == catparser.ast.Array:
            element_type = field_type.element_type
            update_field_type(element_type)
        elif type(field_type) == str:
            pass
        else:
            raise "unexpected"
    
    def constantized_by(field_name, ast_model):
        for x in ast_model.initializers:
            if x.target_property_name == field_name:
                return x.value                    
        return None
            
    def is_size_of_other(field):
        for other_field in ast_model.fields:
            if field.name == other_field.size:
                return other_field
        return None
    
    def skip_in_constructor(field):
        if field.name in ("signature"):
            return True
        return False
    
    def is_member(field):
        if constantized_by(field.name, ast_model):
            return False
        if is_size_of_other(field):
            return False
        if field.is_const:
            return False
        if field.is_reserved:
            return False
        if field.name == "size":
            return False
        return True
    
    def is_method(field):
        return constantized_by(field.name, ast_model)
    
    # prepare
    for f in ast_model.fields:
        update_field_type(f.field_type)
        
    # rust
    ret = ''
    
    # struct
    ret += f'pub struct {ast_model.name} {{'
    for f in ast_model.fields:
        if not is_member(f):
            continue
        if type(f.field_type) == catparser.ast.Array:
            ret += f'pub {f.name}: Vec<{f.field_type.element_type}>,'
        else:
            ret += f'pub {f.name}: {f.field_type},'
    ret += '}'
    
    # implement
    ## const
    ret += 'impl ' + struct_name + ' {'
    for f in ast_model.fields:
        if not f.is_const:
            continue
        if type(f.value) == str:
            ret += f'pub const {f.name}: {f.field_type} = {f.field_type}::{f.value};'
        elif type(f.value) == int:
            ret += f'pub const {f.name}: {f.field_type} = {f.value};'
        else:
            raise "unexpected"
        
    for f in ast_model.fields:
        const = constantized_by(f.name, ast_model)
        if not const:
            continue
        if const in [f.name for f in ast_model.fields]:
            ret += f'pub fn {f.name}(&self) -> {f.field_type} {{ Self::{const} }}'
        else:
            ret += f'pub fn {f.name}(&self) -> {f.field_type} {{ {f.field_type}::default() }}'
        
    ## constructor
    ret += 'pub fn new('
    for f in ast_model.fields:
        if not is_member(f):
            continue
        if skip_in_constructor(f):
            continue
        if type(f.field_type) == catparser.ast.Array:
            ret += f'{f.name}: Vec<{f.field_type.element_type}>,'
        else:
            ret += f'{f.name}: {f.field_type},'
    ret += ') -> Self {'
    ret += 'Self {'
    for f in ast_model.fields:
        if not is_member(f):
            continue
        if skip_in_constructor(f):
            ret += f'{f.name}: {f.field_type}::default(),'
            continue
        ret += f'{f.name},'
    ret += '}'
    ret += '}'

    ret += 'pub fn default() -> Self {'
    ret += 'Self {'
    for f in ast_model.fields:
        if not is_member(f):
            continue
        if type(f.field_type) == catparser.ast.Array:
            ret += f'{f.name}: Vec::new(),'
        elif type(f.value) == catparser.ast.Conditional or f.value is None:
            ret += f'{f.name}: {f.field_type}::default(),'
        else:
            ret += f'{f.name}: {f.value},'
    ret += '}'
    ret += '}'
    
    ## size
    ret += 'pub fn size(&self) -> usize {'
    ret += 'let mut size = 0;'
    for f in ast_model.fields:
        if f.is_const:
            continue
        if f.size is None:
            if is_method(f):
                ret += f'size += self.{f.name}().size();'
            else:
                ret += f'size += self.{f.name}.size();'
        elif type(f.size) == int:
            ret += f'size += {f.size};'
        elif type(f.field_type) == catparser.ast.Array:
            ft = f.field_type
            fte = ft.element_type
            if type(fte) == catparser.ast.FixedSizeInteger:
                ret += f'size += {fte.size} * self.{f.name}.len();'
            elif type(fte) == str:
                ret += f'size += self.{f.name}.iter().map(|x| x.size()).sum::<usize>();'
            else:
                raise "unexpected"
        else:
            raise "unexpected"
    ret += 'size'
    ret += '}'

    ## deserialize
    ret += 'pub fn deserialize(mut payload: &[u8]) -> Result<(Self, &[u8]), SymbolError> {'
    for f in ast_model.fields:
        if f.is_const:
            continue
        fn = f.name
        if constantized_by(f.name, ast_model):
            fn = '_' + fn
        
        ft = f.field_type
        fs = f.size
        if f.name == 'size':
            ret += f'if payload.len() < {fs} {{ return Err(SymbolError::SizeError{{expect: {fs}, real: payload.len()}}) }}'
        if type(ft) == catparser.ast.FixedSizeInteger:
            ret += f'let {fn} = {ft}::from_le_bytes(payload[..{fs}].try_into()?);'
            ret += f'payload = &payload[{fs}..];'
        elif type(ft) == catparser.ast.Array:
            ret += f'let mut {fn} = Vec::new();'
            ret += f'for _ in 0..{fs} {{'
            
            fte = ft.element_type
            if type(fte) == catparser.ast.FixedSizeInteger:
                ftes = fte.size
                # ften = fte.name
                ret += f'let mut bytes = [0u8; {ftes}];'
                ret += f'bytes.copy_from_slice(payload);'
                ret += f'let element = {fte}::from_le_bytes(bytes);'
                ret += f'payload = &payload[{ftes}..];'
                ret += f'{fn}.push(element);'
            elif type(fte) == str:
                ret += f'let element;'
                ret += f'(element, payload) = {fte}::deserialize(payload)?;'
                ret += f'{fn}.push(element);'
            else:
                raise "unexpected"
            ret += '}'
        else:
            ret += f'let {fn};'
            ret += f'({fn}, payload) = {ft}::deserialize(payload)?;'
        
        if f.name == 'size':
            ret += f'if size as usize >= payload.len() + {fs} {{ return Err(SymbolError::SizeError{{expect: size as usize, real: payload.len() + {fs} }}); }}'
        if f.is_reserved:
            ret += f'if {f.name} != 0 {{ return Err(SymbolError::ReservedIsNotZeroError({f.name} as u32)); }}'
        if constantized_by(f.name, ast_model):
            pass
    
    ret += f'let self_ = Self {{'
    for f in ast_model.fields:
        if not is_member(f):
            continue
        ret += f'{f.name},'
    ret += '};'
        
    ret += 'Ok((self_, payload))'
        
    ret += '}'
    
    ## serialize
    ret += 'pub fn serialize(&self) -> Vec<u8> {'
    for f in ast_model.fields:
        if f.is_const:
            continue
        fn = f.name

        ft = f.field_type
        fs = f.size
        if fn == 'size' or constantized_by(f.name, ast_model):
            if type(ft) == catparser.ast.FixedSizeInteger:
                ret += f"let {fn} = self.{fn}().to_le_bytes();"
            else:
                ret += f"let {fn} = self.{fn}().serialize();"
        elif f.is_reserved:
            ret += f'let {fn} = 0{ft.short_name}.to_le_bytes();'
        elif is_size_of_other(f):
            other_field = is_size_of_other(f)
            ofn = other_field.name
            ret += f'let {fn} = self.{ofn}.len().to_le_bytes();'
        elif type(ft) == catparser.ast.Array:
            fte = ft.element_type
            if type(fte) == catparser.ast.FixedSizeInteger:
                ret += f'let {fn}: Vec<u8> = self.{fn}.iter().flat_map(|x| x.to_le_bytes()).collect();'
            else:
                ret += f'let {fn}: Vec<u8> = self.{fn}.iter().flat_map(|x| x.serialize()).collect();'
        elif type(ft) == catparser.ast.FixedSizeInteger:
            ret += f'let {fn} = self.{fn}.to_le_bytes();'
        else:
            ret += f'let {fn} = self.{fn}.serialize();'
            
    ret += '['
    for f in ast_model.fields:
        if f.is_const:
            continue
        fn = f.name
        ret += f'{fn}.iter(),'
    ret += '].into_iter().flat_map(|a| a).map(|x| *x).collect()'
        
    ret += '}'
    ret += '}'
    
    ret = ret.replace('type', 'type_')
    
    ## trait
    for f in ast_model.fields:
        fn = f.name
        if fn not in TRAITS:
            continue
        if type(f.field_type) == catparser.ast.Array:
            ft = f'Vec<{f.field_type.element_type}>'
        else:
            ft = f.field_type
        
        ret += f'impl Trait{snake_to_camel(fn)} for {struct_name} {{'
        ret += f'fn get_{fn}(&self) -> &{ft} {{ &self.{fn} }}'
        ret += f'fn set_{fn}(&mut self, {fn}: {ft}) {{ self.{fn} = {fn}; }}'
        ret += f'}}'
        
    return ret

def generate_enum(ast_model):
    # common (i.e. prepare)
    ret = ''
    name_lower = ast_model.name.lower()
    value_bit_width = ast_model.size * 8
    if value_bit_width not in (8, 16, 32, 64):
        raise 'unexpected'
    value_type = 'u' + str(value_bit_width)

    # structure
    ret += '#[allow(non_camel_case_types)]'
    ret += f'#[repr({value_type})]'
    ret += f'pub enum {ast_model.name} {{'
    ret += ''.join(
        list(
            map(
                lambda e: f'{e.name} = {e.value},',
                ast_model.values,
            )
        )
    )
    if ast_model.is_bitwise:
        ret += f'X({value_type}),'
    ret += '}'

    # implement
    ret += 'impl ' + ast_model.name + ' {'
    
    ## SIZE
    ret += f'pub const SIZE: usize = {ast_model.size};'

    ## constructor
    ret += 'pub fn default() -> Self {'
    ret += f'Self::{ast_model.values[0].name}'
    ret += '}'

    ## size
    ret += 'pub fn size(&self) -> usize {'
    ret += f'Self::SIZE'
    ret += '}'

    ## deserialize
    ret += f'pub fn deserialize(payload: &[u8]) -> Result<(Self, &[u8]), SymbolError> {{'
    ret += 'if payload.len() < Self::SIZE { return Err(SymbolError::SizeError{expect: Self::SIZE, real: payload.len()}) }'
    ret += 'let (bytes, rest) = payload.split_at(Self::SIZE);'
    ret += f'match {value_type}::from_le_bytes(bytes.try_into()?) {{'
    ret += ''.join(
        list(
            map(
                lambda e: f'{e.value} => Ok(({ast_model.name}::{e.name}, rest)),',
                ast_model.values,
            )
        )
    )
    if ast_model.is_bitwise:
        ret += 'x if ('
        ret += ''.join(
            list(
                map(
                    lambda e: f'!{e.value} & ',
                    ast_model.values,
                )
            )
        )
        ret += '!0) == 0 => Ok((Self::X(x), rest)),'
    ret += f'other => Err(SymbolError::MismatchError{{ pattern: vec![ other as u32], place: "{ast_model.name}".to_string() }}),'
    ret += '}'
    ret += '}'


    ## serialize
    if ast_model.is_bitwise:
        ret += f'pub fn serialize(&self) -> Vec<u8> {{'
        ret += 'match self {'
        ret += ''.join(
            list(
                map(
                    lambda e: f'Self::{e.name} => {e.value}{value_type}.to_le_bytes().to_vec(),',
                    ast_model.values,
                )
            )
        )
        ret += 'Self::X(x) => x.to_le_bytes().to_vec(),'
        ret += '}'
        ret += '}'
    else:
        ret += f'pub fn serialize(&self) -> Vec<u8> {{'
        ret += f'(self.clone() as {value_type}).to_le_bytes().to_vec()'
        ret += '}'

    ## to_string
    ret += f'pub fn to_string(&self) -> String {{'
    ret += f'format!("{ast_model.name}::{{:?}}", self)'
    ret += '}'

    # end
    ret += '}'
    
    # BitOr
    if ast_model.is_bitwise:
        pass

    return ret

def generate_bytearray(ast_model):
    # define variables
    ret = ''
    name_lower = ast_model.name.lower()
    value_type = f'[u8; {ast_model.size}]'

    # structure
    ret += f'pub struct {ast_model.name} (pub {value_type});'
    
    # implement
    ret += 'impl ' + ast_model.name + ' {'
    
    ## SIZE
    ret += f'pub const SIZE: usize = {ast_model.size};'

    ## constructor 
    ret += f'pub fn new({name_lower}: {value_type}) -> Self {{'
    ret += f'Self({name_lower})'
    ret += '}'
    
    ret += 'pub fn default() -> Self {'
    ret += f'Self([0; {ast_model.size}])'
    ret += '}'

    ## size
    ret += 'pub fn size(&self) -> usize {'
    ret += 'Self::SIZE'
    ret += '}'

    ## deserialize
    ret += 'pub fn deserialize(payload: &[u8]) -> Result<(Self, &[u8]), SymbolError> {'
    ret += 'if payload.len() < Self::SIZE { return Err(SymbolError::SizeError{expect: Self::SIZE, real: payload.len()}) }'
    ret += 'let (bytes, rest) = payload.split_at(Self::SIZE);'
    ret += 'Ok((Self(bytes.try_into()?), rest))'
    ret += '}'

    ## serialize
    ret += 'pub fn serialize(&self) -> Vec<u8> {'
    ret += 'self.0.to_vec()'
    ret += '}'

    ## to_string
    ret += 'pub fn to_string(&self) -> String {'
    ret += 'format!("0x{}", hex::encode(self.0))'
    ret += '}'

    # end
    ret += '}'

    return ret

def generate_integer(ast_model):
    assert(ast_model.is_unsigned)
    # define variables
    ret = ''
    name_lower = ast_model.name.lower()
    value_bit_width = ast_model.size * 8
    if value_bit_width not in (8, 16, 32, 64):
        raise 'unexpected'
    if ast_model.is_unsigned:
        value_type = 'u' + str(value_bit_width)
    else:
        raise 'unexpected'

    # structure
    ret += f'pub struct {ast_model.name}(pub {value_type});'

    # implement
    ret += 'impl ' + ast_model.name + ' {'
    
    ## SIZE
    ret += f'pub const SIZE: usize = {ast_model.size};'

    ## constructor
    ret += f'pub fn new({name_lower}: {value_type}) -> Self {{'
    ret += f'Self({name_lower})'
    ret += '}'
    
    ret += 'pub fn default() -> Self {'
    ret += 'Self(0)'
    ret += '}'

    ## size
    ret += 'pub fn size(&self) -> usize {'
    ret += 'Self::SIZE'
    ret += '}'

    ## deserialize
    ret += 'pub fn deserialize(payload: &[u8]) -> Result<(Self, &[u8]), SymbolError> {'
    ret += 'if payload.len() < Self::SIZE { return Err(SymbolError::SizeError{expect: Self::SIZE, real: payload.len()}) }'
    ret += 'let (bytes, rest) = payload.split_at(Self::SIZE);'
    ret += f'let value = {value_type}::from_le_bytes(bytes.try_into()?);'
    ret += 'Ok((Self(value), rest))'
    ret += '}'

    ## serialize
    ret += 'pub fn serialize(&self) -> Vec<u8> {'
    ret += 'self.0.to_le_bytes().to_vec()'
    ret += '}'

    ## to_string
    ret += 'pub fn to_string(&self) -> String {'
    ret += f'format!("0x{{:0{ast_model.size * 2}x}}", self.0)'
    ret += '}'

    # end
    ret += '}'

    return ret

def header_for_each_ast_model(ast_model):
    ret = ""
    ret += display_ast_model(ast_model)
    ret += '#[derive(Debug, Clone, PartialEq, Eq)]\n'
    return ret

def display_ast_model(obj, indent: int = 0):
    ret = ""
    prefix = '//' + '  ' * indent
    if hasattr(obj, '__dict__'):
        for key, value in vars(obj).items():
            if key == 'comment':
                if indent != 0:
                    continue
                if str(value) == 'None':
                    ret += f'\n'
                    continue
                value = str(value).replace("\n", "\n///")
                ret += f'\n///{value}\n'
                continue
            if key.startswith("_"):
                continue
            if hasattr(value, '__dict__') or type(value) == list:
                if issubclass(type(value), Enum):
                    ret += f'{prefix}{key}: {value}\n'
                else:
                    ret += f'{prefix}{key}: {type(value)}\n'
                ret += display_ast_model(value, indent + 2)
                continue
            ret += f'{prefix}{key}: {value}\n'
            
    if type(obj) == list:
        for element in obj:
            if hasattr(element, '__dict__') or type(element) == list:
                if "\n" in str(element):
                    tmp = str(element).replace("\n", " ")
                else:
                    tmp = element
                ret += f'{prefix}{tmp}\n'
                ret += display_ast_model(element, indent + 2)
                continue
            ret += f'{prefix}{element}\n'
            
    for method_name in dir(type(obj)):
        if method_name.startswith("_"):
            continue
        if method_name == "to_legacy_descriptor":
            continue
        method_res = getattr(obj, method_name)
        if "built-in" in str(method_res):
            continue
        if "DisplayType" in str(method_res):
            continue
        if "<" in str(method_res):
            continue
        ret += f'{prefix}*{method_name}: {method_res}\n'
    
    return ret