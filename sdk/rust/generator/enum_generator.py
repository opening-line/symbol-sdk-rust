from pathlib import Path
from enum import Enum
import catparser
from catparser.DisplayType import DisplayType

def generate_enum(ast_model):
    # common (i.e. prepare)
    ret = '// generated from generate_enum()\n'
    name_lower = ast_model.name.lower()
    value_bit_width = ast_model.size * 8
    if value_bit_width not in (8, 16, 32, 64):
        raise 'unexpected'
    value_type = 'u' + str(value_bit_width)

    # structure
    ret += '#[allow(non_camel_case_types)]'
    ret += f'#[repr({value_type})]'
    if ast_model.name == 'NetworkType':
        ret += '#[cfg(not(feature = "private_network"))]'
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
    if ast_model.name == 'NetworkType':
        ret += '#[cfg(not(feature = "private_network"))]'
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
    ret += 'if payload.len() < Self::SIZE { return Err(SymbolError::SizeError{expect: vec![Self::SIZE], real: payload.len()}) }'
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

    # end
    ret += '}'
    
    # BitOr
    if ast_model.is_bitwise:
        pass

    return ret
