from pathlib import Path
from enum import Enum
import catparser
from generator import util

def generate_factory(factory, products):
    factory_name = factory.name
    ret = '// generated from generate_factory()\n'
    
    # enum
    ret += f'pub enum {factory_name} {{'
    for p in products:
        ret += f'{p.name}({p.name}),'
    ret += '}'
    
    ret += '#[allow(unreachable_patterns)]'
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
        if util.constantized_by(fn, factory):
            if fn not in factory.discriminator:
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
        if util.constantized_by(f.name, factory):
            pass
        
    common_field_name_list = [f.name for f in factory.fields]
    ret += 'match ('
    for d in factory.discriminator:
        ret += f'{d}, '
    ret += ') {'
    for p in products:
        ret += '('
        for d in factory.discriminator:
            ret += f'{p.name}::{util.constantized_by(d, p)}, '
        ret += ') => {'
        for f in p.fields:
            if f.name in common_field_name_list:
                continue
            if f.is_const:
                continue
            fn = f.name
            if util.constantized_by(f.name, p):
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
            if util.constantized_by(f.name, factory):
                pass
        ret += f'let self_ = {p.name} {{'
        for f in p.fields:
            if not util.is_member(f, p):
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
    
    