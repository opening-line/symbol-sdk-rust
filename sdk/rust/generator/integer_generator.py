from pathlib import Path
from enum import Enum
import catparser
from catparser.DisplayType import DisplayType

def generate_integer(ast_model):
    assert(ast_model.is_unsigned)
    # define variables
    ret = '// generated from generate_integer()\n'
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

    # end
    ret += '}'

    return ret