#!/usr/bin/python

# Since the Rust language does not support default arguments,
# the new(args) and default() functions are provided as constructors.
# new(args): requires an argument
# default(): no argument required, default value is set

from pathlib import Path

from catparser.DisplayType import DisplayType
from catparser.generators.util import build_factory_map

from .FactoryFormatter import FactoryClassFormatter, FactoryFormatter

from .format import indent

class Generator:
	@staticmethod
	def generate(ast_models, output):
		print(f'python catbuffer generator called with output: {output}')
		generate_files(ast_models, Path(output))


def generate_files(ast_models, output_directory: Path):
	factory_map = build_factory_map(ast_models)

	output_directory.mkdir(exist_ok=True)

	with open(output_directory / 'rust.txt', 'w', encoding='utf8', newline='\n') as output_file:
		output_file.write(
			'''// rust
use hex;

'''
		)
		import copy
		output = ''
		for ast_model in ast_models:
			ast_model = copy.deepcopy(ast_model)
			if ast_model.display_type == DisplayType.STRUCT:
				output += generate_struct(ast_model)
			elif ast_model.display_type == DisplayType.ENUM:
				output += generate_enum(ast_model)
			elif ast_model.display_type == DisplayType.BYTE_ARRAY:
				output += generate_bytearray(ast_model)
			elif ast_model.display_type == DisplayType.INTEGER:
				output += generate_integer(ast_model)
			else:
				raise 'Unexpected'

		# for ast_model in ast_models:
		# 	if DisplayType.STRUCT == ast_model.display_type and ast_model.is_abstract:
		# 		factory_generator = FactoryClassFormatter(FactoryFormatter(factory_map, ast_model))
		# 		output += str(factory_generator)

		output_file.write(output)

def generate_struct(ast_model):
	# self.name = _get_token_value(tokens[1])
	# self.disposition = _get_token_value(tokens[0]) if tokens[0] else None
	# self.fields = tokens[2:]
	# self.factory_type = None
	# self.display_type = DisplayType.STRUCT
	# self.attributes = None
	# self.requires_unaligned = False
	# self._member_comment_start_regex = None

	# common (i.e. prepare)
	import re

	def is_int(s):
		try:
			int(s, 10)
		except ValueError:
			return False
		else:
			return True
		
	count_list_for_skip = []
	ref_const_dict = {}
	for field in ast_model.fields:
		if field.disposition == 'const':
			key = field.name.split('_')[1].lower()
			if key == 'type':
				key = 'type_'
			ref_const_dict[key] = f"Self::{field.name}"
		field.field_type = re.sub(r'uint(8|16|32|64)', r'u\1', str(field.field_type))
		field.field_type = re.sub(r'int(8|16|32|64)', r'i\1', str(field.field_type))
		if field.name == "type":
			field.name = "type_"
		m = re.match(r'^array\((\w*), (\w*)\)', str(field.field_type))
		if m is not None:
			count_list_for_skip.append(m.group(2))
			print(m.group(0))
			field.field_type = f'Vec<{m.group(1)}>'
	for field in ast_model.fields:
		if field.value is None:
			if field.name in ref_const_dict:
				field.value = ref_const_dict[field.name]
			elif field.field_type.startswith("Vec"):
				field.value = 'Vec::new()'
			elif field.field_type in ("i8", "i16", "i32", "i64", "u8", "u16", "u32", "u64"):
				field.value = 0
			else:
				field.value = f'{field.field_type}::default()'
		elif 'equals' in str(field.value) and 'if' in str(field.value):
			field.value = f'{field.field_type}::default()'
	fields = {}
				
	ret = ''

	# anotation
	print("\n\nname: size == ", len(ast_model.fields))
	ret += '/// ast_model.display_type == DisplayType.STRUCT\n'
	
	print("class", ast_model.name)
	for field in ast_model.fields:
		print("name", field.name)
		print("field_type", field.field_type)
		print("value", field.value)
		print("disposition", field.disposition)
	print()

	# structure
	ret += '#[derive(Debug)]\n'
	ret += f'pub struct {ast_model.name} {{\n'
	for field in ast_model.fields:
		if field.disposition == 'const':
			continue
		if field.name in count_list_for_skip:
			continue
		if field.name == 'size':
			continue
		ret += f'\tpub {field.name}: {field.field_type},\n'
	ret += '}\n'

	# implement
	ret += 'impl ' + ast_model.name + ' {\n'
	for field in ast_model.fields:
		if field.disposition != 'const':
			continue
		if field.value is None:
			value = field.field_type + "::default()"
		elif is_int(str(field.value)):
			value = field.field_type
		else:
			# value = 
			pass
		if field.field_type in ("i8", "i16", "i32", "i64", "u8", "u16", "u32", "u64"):
			ret += f'\tconst {field.name}: {field.field_type} = {field.value};\n'
		else:
			ret += f'\tconst {field.name}: {field.field_type} = {field.field_type}::{field.value};\n'
	## constructor
	ret += '\tpub fn new() -> Self {\n'
	ret += '\t\tSelf {\n'
	for field in ast_model.fields:
		if field.disposition == 'const':
			continue
		if field.value is None:
			continue
		if field.name in count_list_for_skip:
			continue
		if field.name == 'size':
			continue
		ret += f'\t\t\t{field.name}: {field.value},\n'
	ret += '\t\t}\n'
	ret += '\t}\n'

	ret += '\tpub fn default() -> Self {\n'
	ret += '\t\tSelf::new()\n'
	ret += '\t}\n'

	## size
	ret += '\tpub fn size(&self) -> usize {\n'
	ret += '\t\tlet mut size = 0;\n'
	for field in ast_model.fields:
		if field.disposition == 'const':
			continue
		if field.field_type in ("i8", "i16", "i32", "i64", "u8", "u16", "u32", "u64"):
			size = int(field.field_type[1:]) // 8
		elif field.field_type.startswith("Vec"):
			size = f'self.{field.name}.len()'
		else:
			size = f'{field.field_type}::SIZE'
		ret += f'\t\tsize += {size};\n'
	ret += '\t\tsize\n'
	ret += '\t}\n'

	## deserialize
	ret += '\tpub fn deserialize(payload: &[u8]) -> Self {\n'
	ret += '\t\tlet size = u32::from_le_bytes(payload[..4].clone());\n'
	ret += '\t\tassert_eq!(size, payload.len());\n'
	ret += '\t\tlet payload = &payload[4..];'
	for field in ast_model.fields:
		if field.disposition == 'const':
			continue
		if field.field_type in ("i8", "i16", "i32", "i64", "u8", "u16", "u32", "u64"):
			size = int(field.field_type[1:]) // 8
		elif field.field_type.startswith("Vec"):
			size = f'self.{field.name}.len()'
		else:
			size = f'{field.field_type}::SIZE'
		if field.disposition == 'reserved':
			pass
	ret += '\t}\n'

	# ## serialize
	# ret += f'\tpub fn serialize(&self) -> [u8; ] {{\n'
	# ret += '\t\tself.value.to_le_bytes()\n'
	# ret += '\t}\n'

	# ## to_string
	# ret += f'\tpub fn to_string(&self) -> String {{\n'
	# ret += '\t}\n'

	# end
	ret += '}\n\n'

	return ret + '\n'

def generate_enum(ast_model):
	# common (i.e. prepare)
	ret = ''
	name_lower = ast_model.name.lower()
	value_bit_width = ast_model.size * 8
	if value_bit_width not in (8, 16, 32, 64):
		raise 'unexpected'
	value_type = 'u' + str(value_bit_width)

	# anotation
	ret += '/// ast_model.display_type == DisplayType.ENUM\n'

	# structure
	ret += '#[derive(Debug, Clone)]\n'
	ret += '#[allow(non_camel_case_types)]\n'
	ret += f'pub enum {ast_model.name} {{\n'
	ret += ''.join(
		list(
			map(
				lambda e: f'\t{e.name} = {e.value},\n',
				ast_model.values,
			)
		)
	)
	ret += '}\n'

	# implement
	ret += 'impl ' + ast_model.name + ' {\n'
	## SIZE # or size
	ret += f'\tconst SIZE: usize = {ast_model.size};\n'

	## constructor
	ret += '\tpub fn default() -> Self {\n'
	ret += f'\t\tSelf::{ast_model.values[0].name}\n'
	ret += '\t}\n'

	## size
	ret += '\tpub fn size(&self) -> usize {\n'
	ret += f'\t\tSelf::SIZE\n'
	ret += '\t}\n'

	## deserialize
	ret += f'\tpub fn deserialize(payload: &[u8; {ast_model.size}]) -> Option<Self> {{\n'
	ret += f'\t\tmatch {value_type}::from_le_bytes(payload.clone()) {{\n'
	ret += ''.join(
		list(
			map(
				lambda e: f'\t\t\t{e.value} => Some({ast_model.name}::{e.name}),\n',
				ast_model.values,
			)
		)
	)
	ret += '\t\t\t_ => None,\n'
	ret += '\t\t}\n'
	ret += '\t}\n'

	## serialize
	ret += f'\tpub fn serialize(&self) -> [u8; {ast_model.size}] {{\n'
	ret += f'\t\t(self.clone() as {value_type}).to_le_bytes()\n'
	ret += '\t}\n'

	## to_string
	ret += f'\tpub fn to_string(&self) -> String {{\n'
	ret += f'\t\tformat!("{ast_model.name}::{{:?}}", self)\n'
	ret += '\t}\n'

	# end
	ret += '}\n\n'

	return ret

def generate_bytearray(ast_model):
	# common (i.e. prepare)
	ret = ''
	name_lower = ast_model.name.lower()
	value_type = f'[u8; {ast_model.size}]'

	# anotation
	ret += '/// ast_model.display_type == DisplayType.BYTE_ARRAY\n'

	# structure
	ret += '#[derive(Debug, Clone)]\n'
	ret += f'pub struct {ast_model.name} {{\n'
	ret += f'\tbytes: {value_type}\n'
	ret += '}\n'
	
	# implement
	ret += 'impl ' + ast_model.name + ' {\n'
	## SIZE # or size
	ret += f'\tconst SIZE: usize = {ast_model.size};\n'

	## constructor 
	ret += f'\tpub fn new({name_lower}: {value_type}) -> Self {{\n'
	ret += '\t\tSelf {\n'
	ret += f'\t\t\tbytes: {name_lower}\n'
	ret += '\t\t}\n'
	ret += '\t}\n'
	ret += '\tpub fn default() -> Self {\n'
	ret += f'\t\tSelf::new([0; {ast_model.size}])\n'
	ret += '\t}\n'

	## size
	ret += '\tpub fn size(&self) -> usize {\n'
	ret += f'\t\tSelf::SIZE\n'
	ret += '\t}\n'

	## deserialize
	ret += f'\tpub fn deserialize({name_lower}: &{value_type}) -> Self {{\n'
	ret += f'\t\tSelf::new({name_lower}.clone())\n'
	ret += '\t}\n'

	## serialize
	ret += f'\tpub fn serialize(&self) -> [u8; {ast_model.size}] {{\n'
	ret += '\t\tself.bytes\n'
	ret += '\t}\n'

	## to_string
	ret += f'\tpub fn to_string(&self) -> String {{\n'
	ret += f'\t\t"0x".to_string() + &hex::encode(self.bytes)\n'
	ret += '\t}\n'

	# end
	ret += '}\n\n'

	return ret

def generate_integer(ast_model):
	# tip: print(ast_model.is_unsigned) # all unsigned
	# common (i.e. prepare)
	ret = ''
	name_lower = ast_model.name.lower()
	value_bit_width = ast_model.size * 8
	if value_bit_width not in (8, 16, 32, 64):
		raise 'unexpected'
	value_type = 'u' + str(value_bit_width)

	# anotation
	ret += '/// ast_model.display_type == DisplayType.INTEGER\n'

	# structure
	ret += '#[derive(Debug, Clone)]\n'
	ret += f'pub struct {ast_model.name} {{\n'
	ret += f'\tvalue: {value_type}\n'
	ret += '}\n'

	# implement
	ret += 'impl ' + ast_model.name + ' {\n'
	## SIZE # or size
	ret += f'\tconst SIZE: usize = {ast_model.size};\n'

	## constructor
	ret += f'\tpub fn new({name_lower}: {value_type}) -> Self {{\n'
	ret += '\t\tSelf {\n'
	ret += f'\t\t\tvalue: {name_lower}\n'
	ret += '\t\t}\n'
	ret += '\t}\n'
	ret += '\tpub fn default() -> Self {\n'
	ret += f'\t\tSelf::new(0)\n'
	ret += '\t}\n'

	## size
	ret += '\tpub fn size(&self) -> usize {\n'
	ret += f'\t\tSelf::SIZE\n'
	ret += '\t}\n'

	## deserialize
	ret += f'\tpub fn deserialize(payload: &[u8; {ast_model.size}]) -> Self {{\n'
	ret += f'\t\tSelf::new({value_type}::from_le_bytes(payload.clone()))\n'
	ret += '\t}\n'

	## serialize
	ret += f'\tpub fn serialize(&self) -> [u8; {ast_model.size}] {{\n'
	ret += '\t\tself.value.to_le_bytes()\n'
	ret += '\t}\n'

	## to_string
	ret += f'\tpub fn to_string(&self) -> String {{\n'
	ret += f'\t\tformat!("0x{{:0{ast_model.size * 2}x}}", self.value)\n'
	ret += '\t}\n'

	# end
	ret += '}\n\n'

	return ret
