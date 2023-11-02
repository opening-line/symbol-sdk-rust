#!/usr/bin/python

# Since the Rust language does not support default arguments,
# the new(args) and default() functions are provided as constructors.
# new(args): requires an argument
# default(): no argument required, default value is set

# todo: to_string()
# todo: factory

from pathlib import Path
from enum import Enum
import catparser
import lark
from catparser.DisplayType import DisplayType
# from catparser.generators.util import build_factory_map
# from .FactoryFormatter import FactoryClassFormatter, FactoryFormatter

# from .format import indent

class Generator:
    @staticmethod
    def generate(ast_models, output):
        print(f'python catbuffer generator called with output: {output}')
        generate_files(ast_models, Path(output))


def generate_files(ast_models, output_directory: Path):
    # factory_map = build_factory_map(ast_models)

    output_directory.mkdir(exist_ok=True)

    with open(output_directory / 'models.rs', 'w', encoding='utf8', newline='') as output_file:
        import copy
        output = ''
        with open("generator/models_header.rs", "r") as f:
            output += f.read()
        for ast_model in ast_models:
            ast_model = copy.deepcopy(ast_model)
            output += header_for_each_ast_model(ast_model)
            if ast_model.display_type == DisplayType.STRUCT:
                output += generate_struct2(ast_model)
            elif ast_model.display_type == DisplayType.ENUM:
                output += generate_enum(ast_model)
            elif ast_model.display_type == DisplayType.BYTE_ARRAY:
                output += generate_bytearray(ast_model)
            elif ast_model.display_type == DisplayType.INTEGER:
                output += generate_integer(ast_model)
            else:
                raise 'Unexpected'

        # for ast_model in ast_models:
        #     if DisplayType.STRUCT == ast_model.display_type and ast_model.is_abstract:
        #         struct_name = ast_model.name
        #         fields = ast_model.fields
        #         print("##" + struct_name)
        #         print(ast_model)
        #         for f in fields:
        #             print(f.name)
        #         factory_generator = FactoryClassFormatter(FactoryFormatter(factory_map, ast_model))
        #         output += str(factory_generator)

        output_file.write(output)

def generate_struct2(ast_model):
    struct_name = ast_model.name
    
    if struct_name == "EmbeddedTransferTransactionV1":
        display_ast_model(ast_model.initializers)
    
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
    
    def is_constantized(field):
        for x in ast_model.initializers:
            if x.target_property_name == field.name:
                return x
        return None

    def constantized_by(field):
        for x in ast_model.initializers:
            if x.target_property_name == field.name:
                return x.value
            
    def is_size_of_other(field):
        for other_field in ast_model.fields:
            if field.name == other_field.size:
                return other_field
        return None
    
    def is_member(field):
        if is_constantized(field):
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
        method_list = ["version", "type"]
        return field.name in method_list
        
    
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
            ret += f'const {f.name}: {f.field_type} = {f.field_type}::{f.value};'
        elif type(f.value) == int:
            ret += f'const {f.name}: {f.field_type} = {f.value};'
        else:
            raise "unexpected"
        
    for f in ast_model.fields:
        if not is_constantized(f):
            continue
        const = constantized_by(f)
        if const in [f.name for f in ast_model.fields]:
            ret += f'fn {f.name}(&self) -> {f.field_type} {{ Self::{const} }}'
        else:
            ret += f'fn {f.name}(&self) -> {f.field_type} {{ {f.field_type}::default() }}'
        
    ## constructor
    ret += 'pub fn new('
    for f in ast_model.fields:
        if not is_member(f):
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
    ret += 'pub fn deserialize(mut payload: &[u8]) -> Option<(Self, &[u8])> {'
    for f in ast_model.fields:
        if f.is_const:
            continue
        fn = f.name
        if is_constantized(f):
            fn = '_' + fn
        
        ft = f.field_type
        fs = f.size
        if type(ft) == catparser.ast.FixedSizeInteger:
            ret += f'let {fn} = {ft}::from_le_bytes(payload[..{fs}].try_into().ok()?);'
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
            ret += f'if size as usize >= payload.len() + {fs} {{'
            ret += 'return None;'
            ret += '}'
        if f.is_reserved:
            ret += f'if {f.name} != 0 {{'
            ret += 'return None;'
            ret += '}'
        if is_constantized(f):
            pass
    
    ret += f'let self_ = Self {{'
    for f in ast_model.fields:
        if not is_member(f):
            continue
        ret += f'{f.name},'
    ret += '};'
        
    ret += 'Some((self_, payload))'
        
    ret += '}'
    
    ## serialize
    ret += 'pub fn serialize(&self) -> Vec<u8> {'
    for f in ast_model.fields:
        if f.is_const:
            continue
        fn = f.name

        ft = f.field_type
        fs = f.size
        if fn == 'size' or is_constantized(f):
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
        # elif is_size_of_other(f):
        #     continue
        fn = f.name
        ret += f'{fn}.iter(),'
    ret += '].into_iter().flat_map(|a| a).map(|x| *x).collect()'
        
    ret += '}'
    
    
    ret += '}'
    return ret.replace('type', 'type_')

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
    ret += f'pub enum {ast_model.name} {{'
    ret += ''.join(
        list(
            map(
                lambda e: f'{e.name} = {e.value},',
                ast_model.values,
            )
        )
    )
    ret += '}'

    # implement
    ret += 'impl ' + ast_model.name + ' {'
    ## SIZE # or size
    ret += f'const SIZE: usize = {ast_model.size};'

    ## constructor
    ret += 'pub fn default() -> Self {'
    ret += f'Self::{ast_model.values[0].name}'
    ret += '}'

    ## size
    ret += 'pub fn size(&self) -> usize {'
    ret += f'Self::SIZE'
    ret += '}'

    ## deserialize
    ret += f'pub fn deserialize(payload: &[u8]) -> Option<(Self, &[u8])> {{'
    ret += f'if payload.len() < {ast_model.size} {{ return None; }}'
    ret += f'let mut bytes = [0u8; {ast_model.size}];'
    ret += f'bytes.copy_from_slice(payload);'
    ret += f'match {value_type}::from_le_bytes(bytes) {{'
    ret += ''.join(
        list(
            map(
                lambda e: f'{e.value} => Some(({ast_model.name}::{e.name}, &payload[{ast_model.size}..])),',
                ast_model.values,
            )
        )
    )
    ret += '_ => None,'
    ret += '}'
    ret += '}'

    ## serialize
    ret += f'pub fn serialize(&self) -> Vec<u8> {{'
    ret += f'(self.clone() as {value_type}).to_le_bytes().to_vec()'
    ret += '}'

    ## to_string
    ret += f'pub fn to_string(&self) -> String {{'
    ret += f'format!("{ast_model.name}::{{:?}}", self)'
    ret += '}'

    # end
    ret += '}'

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
    ## SIZE # or size
    ret += f'const SIZE: usize = {ast_model.size};'

    ## constructor 
    ret += f'pub fn new({name_lower}: {value_type}) -> Self {{'
    ret += f'Self({name_lower})'
    ret += '}'
    
    ret += 'pub fn default() -> Self {'
    ret += f'Self([0; {ast_model.size}])'
    ret += '}'
    
    ret += 'pub fn from_str(hex_str: &str) -> Option<Self> {'
    ret += f'let mut bytes = [0; {ast_model.size}];'
    ret += 'if let Err(_) = hex::decode_to_slice(hex_str, &mut bytes) {'
    ret += 'None'
    ret += '} else {'
    ret += 'Some(Self::new(bytes))'
    ret += '}'
    ret += '}'

    ## size
    ret += 'pub fn size(&self) -> usize {'
    ret += f'Self::SIZE'
    ret += '}'

    ## deserialize
    ret += f'pub fn deserialize(payload: &[u8]) -> Option<(Self, &[u8])> {{'
    ret += f'if payload.len() < {ast_model.size} {{ return None; }}'
    ret += f'let mut bytes = [0u8; {ast_model.size}];'
    ret += f'bytes.copy_from_slice(payload);'
    ret += f'Some((Self::new(bytes), &payload[..{ast_model.size}]))'
    ret += '}'

    ## serialize
    ret += f'pub fn serialize(&self) -> Vec<u8> {{'
    ret += 'self.0.to_vec()'
    ret += '}'

    ## to_string
    ret += f'pub fn to_string(&self) -> String {{'
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
    ## SIZE # or size
    ret += f'const SIZE: usize = {ast_model.size};'

    ## constructor
    ret += f'pub fn new({name_lower}: {value_type}) -> Self {{'
    ret += f'Self({name_lower})'
    ret += '}'
    
    ret += 'pub fn default() -> Self {'
    ret += f'Self(0)'
    ret += '}'

    ## size
    ret += 'pub fn size(&self) -> usize {'
    ret += f'Self::SIZE'
    ret += '}'

    ## deserialize
    ret += f'pub fn deserialize(payload: &[u8]) -> Option<(Self, &[u8])> {{'
    ret += f'if payload.len() < {ast_model.size} {{ return None; }}'
    ret += f'let mut bytes = [0u8; {ast_model.size}];'
    ret += f'bytes.copy_from_slice(payload);'
    ret += f'let value = {value_type}::from_le_bytes(bytes);'
    ret += f'Some((Self::new(value), &payload[..{ast_model.size}]))'
    ret += '}'

    ## serialize
    ret += f'pub fn serialize(&self) -> Vec<u8> {{'
    ret += 'self.0.to_le_bytes().to_vec()'
    ret += '}'

    ## to_string
    ret += f'pub fn to_string(&self) -> String {{'
    ret += f'format!("0x{{:0{ast_model.size * 2}x}}", self.0)'
    ret += '}'

    # end
    ret += '}'

    return ret

def header_for_each_ast_model(ast_model):
    ret = ""
    ret += display_ast_model(ast_model)
    ret += '#[derive(Debug, Clone, PartialEq, PartialOrd)]\n'
    return ret
def display_ast_model(obj, indent: int = 0):
    ret = ""
    prefix = '///' + '  ' * indent
    if hasattr(obj, '__dict__'):
        for key, value in vars(obj).items():
            if key == 'comment':
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
                element_out = element
                if "\n" in str(element):
                    element_out = str(element).replace("\n", " ")
                ret += f'{prefix}{element_out}\n'
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