from pathlib import Path
from enum import Enum
import catparser
from catparser.DisplayType import DisplayType

# from generator import const
# from generator import util

def generate_bytearray(ast_model):
    # define variables
    ret = '// generated from generate_bytearray()\n'
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
    
    # end
    ret += '}'

    return ret