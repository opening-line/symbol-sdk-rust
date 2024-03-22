from pathlib import Path
from enum import Enum
import catparser
from catparser.DisplayType import DisplayType
from generator import constant
from generator import util


def generate_struct(ast_model):
    struct_name = ast_model.name
    ret = '// generated from generate_struct()\n'
    
    # struct
    ret += f'pub struct {ast_model.name} {{'
    for f in ast_model.fields:
        if not util.is_member(f, ast_model):
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
        const = util.constantized_by(f.name, ast_model)
        if not const:
            continue
        if const in [f.name for f in ast_model.fields]:
            ret += f'pub fn {f.name}(&self) -> {f.field_type} {{ Self::{const} }}'
        else:
            ret += f'pub fn {f.name}(&self) -> {f.field_type} {{ {f.field_type}::default() }}'
        
    ## constructor
    ret += 'pub fn new('
    for f in ast_model.fields:
        if not util.is_member(f, ast_model):
            continue
        if util.skip_in_constructor(f):
            continue
        if type(f.field_type) == catparser.ast.Array:
            ret += f'{f.name}: Vec<{f.field_type.element_type}>,'
        else:
            ret += f'{f.name}: {f.field_type},'
    ret += ') -> Self {'
    ret += 'Self {'
    for f in ast_model.fields:
        if not util.is_member(f, ast_model):
            continue
        if util.skip_in_constructor(f):
            ret += f'{f.name}: {f.field_type}::default(),'
            continue
        ret += f'{f.name},'
    ret += '}'
    ret += '}'

    ret += 'pub fn default() -> Self {'
    ret += 'Self {'
    for f in ast_model.fields:
        if not util.is_member(f, ast_model):
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
            if util.is_method(f, ast_model):
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
        if util.constantized_by(f.name, ast_model):
            fn = '_' + fn
        
        ft = f.field_type
        fs = f.size
        if f.name == 'size':
            ret += f'if payload.len() < {fs} {{ return Err(SymbolError::SizeError{{expect: vec![{fs}], real: payload.len()}}) }}'
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
            ret += f'if size as usize >= payload.len() + {fs} {{ return Err(SymbolError::SizeError{{expect: vec![size as usize], real: payload.len() + {fs} }}); }}'
        if f.is_reserved:
            ret += f'if {f.name} != 0 {{ return Err(SymbolError::ReservedIsNotZeroError({f.name} as u32)); }}'
        if util.constantized_by(f.name, ast_model):
            pass
    
    ret += f'let self_ = Self {{'
    for f in ast_model.fields:
        if not util.is_member(f, ast_model):
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
        if fn == 'size' or util.constantized_by(f.name, ast_model):
            if type(ft) == catparser.ast.FixedSizeInteger:
                ret += f"let {fn} = self.{fn}().to_le_bytes();"
            else:
                ret += f"let {fn} = self.{fn}().serialize();"
        elif f.is_reserved:
            ret += f'let {fn} = 0{ft.short_name}.to_le_bytes();'
        elif util.is_size_of_other(f, ast_model):
            other_field = util.is_size_of_other(f, ast_model)
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
        if fn not in constant.TRAITS:
            continue
        if type(f.field_type) == catparser.ast.Array:
            ft = f'Vec<{f.field_type.element_type}>'
        else:
            ft = f.field_type
        
        ret += f'impl Trait{util.snake_to_camel(fn)} for {struct_name} {{'
        ret += f'fn get_{fn}(&self) -> &{ft} {{ &self.{fn} }}'
        ret += f'fn set_{fn}(&mut self, {fn}: {ft}) {{ self.{fn} = {fn}; }}'
        ret += f'}}'
        
    return ret