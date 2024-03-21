from pathlib import Path
from enum import Enum
import catparser
from catparser.DisplayType import DisplayType

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

def update_int_type_of_struct(ast_models):
    def _update_int_type(field_type):
        if type(field_type) == catparser.ast.FixedSizeInteger:
            field_type.short_name = field_type.short_name.replace("uint", "u").replace("int", "i")
        elif type(field_type) == catparser.ast.Array:
            element_type = field_type.element_type
            _update_int_type(element_type)
        else:
            pass
    for ast_model in ast_models:
        if ast_model.display_type != DisplayType.STRUCT:
            continue
        for f in ast_model.fields:
            _update_int_type(f.field_type)
            
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

def is_method(field, ast_model):
    return constantized_by(field.name, ast_model)

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