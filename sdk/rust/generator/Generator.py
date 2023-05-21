#!/usr/bin/python

# Since the Rust language does not support default arguments,
# the new(args) and default() functions are provided as constructors.
# new(args): requires an argument
# default(): no argument required, default value is set

from pathlib import Path

from catparser.DisplayType import DisplayType
from catparser.generators.util import build_factory_map, extend_models

from .FactoryFormatter import FactoryClassFormatter, FactoryFormatter

from .format import indent

class Generator:
	@staticmethod
	def generate(ast_models, output):
		print(f'python catbuffer generator called with output: {output}')
		generate_files(ast_models, Path(output))


# def create_printer(descriptor, name, is_pod):
# 	return (create_pod_printer if is_pod else BuiltinPrinter)(descriptor, name)

def generate_files(ast_models, output_directory: Path):
	factory_map = build_factory_map(ast_models)
	# extend_models(ast_models, create_printer)

	output_directory.mkdir(exist_ok=True)

	with open(output_directory / 'rust.txt', 'w', encoding='utf8', newline='\n') as output_file:
		output_file.write(
			'''// rust
use hex;

'''
		)
		output = ''
		for ast_model in ast_models:
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

		for ast_model in ast_models:
			if DisplayType.STRUCT == ast_model.display_type and ast_model.is_abstract:
				factory_generator = FactoryClassFormatter(FactoryFormatter(factory_map, ast_model))
				output += str(factory_generator)

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
	for field in ast_model.fields:
		field.field_type = re.sub(r'^int(8|16|32|64)$', r'i\1', str(field.field_type))
		field.field_type = re.sub(r'^uint(8|16|32|64)$', r'u\1', str(field.field_type))
		# field.field_type = re.sub(r'^array.*$', r'Vec', str(field.field_type))
	ret = ''

	# anotation
	ret += '/// ast_model.display_type == DisplayType.STRUCT\n'
	
	# print()
	# print(ast_model.name)
	# for field in ast_model.fields:
	# 	tmp = field
	# 	tmp.field_type = re.sub(r'^int(8|16|32|64)$', r'i\1', str(tmp.field_type))
	# 	tmp.field_type = re.sub(r'^uint(8|16|32|64)$', r'u\1', str(tmp.field_type))
	# 	print("0", tmp.value)
	# 	if tmp.value is None:
	# 		pass
			# tmp.value = str(tmp.field_type) + "::default()"
			# print("1", tmp.value)

	# 	elif is_int(str(filed.value)):
	# 		continue
	# 	else:
	# 		print("3", str(filed.value))
	# 		# filed.value = f"?{str(filed.value)}"
	# 		print("3", str(filed.value))
	# 		continue
	
	print("class", ast_model.name)
	for i in ast_model.fields:
		print("name", i.name)
		print("field_type", i.field_type)
		print("value", i.value)
		print("disposition", i.disposition)
	print()

	# # structure
	ret += '#[derive(Debug)]\n'
	ret += f'pub struct {ast_model.name} {{\n'
	for field in ast_model.fields:
		if field.disposition == 'const':
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
		ret += f'\tconst {field.name}: {field.field_type} = '
	## SIZE # or size

	# ## constructor
	# ret += f'\tpub fn new({name_lower}: {value_type}) -> Self {{\n'
	# ret += '\t\tSelf {\n'
	# ret += f'\t\t\tvalue: {name_lower}\n'
	# ret += '\t\t}\n'
	# ret += '\t}\n'
	# ret += '\tpub fn default() -> Self {\n'
	# ret += f'\t\tSelf::new(0)\n'
	# ret += '\t}\n'

	# ## size
	# ret += '\tpub fn size(&self) -> usize {\n'
	# ret += f'\t\tSelf::SIZE\n'
	# ret += '\t}\n'

	# ## deserialize
	# ret += f'\tpub fn deserialize(payload: ) -> Self {{\n'
	# ret += f'\t\tSelf::new({value_type}::from_le_bytes(payload))\n'
	# ret += '\t}\n'

	# ## serialize
	# ret += f'\tpub fn serialize(&self) -> [u8; ] {{\n'
	# ret += '\t\tself.value.to_le_bytes()\n'
	# ret += '\t}\n'

	# ## to_string
	# ret += f'\tpub fn to_string(&self) -> String {{\n'
	# ret += '\t}\n'

	# # end
	# ret += '}\n\n'

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
	ret += '#[derive(Debug)]\n'
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
	ret += f'\tpub fn deserialize(payload: [u8; {ast_model.size}]) -> Option<Self> {{\n'
	ret += f'\t\tmatch {value_type}::from_le_bytes(payload) {{\n'
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
	ret += f'\t\t(*self as {value_type}).to_le_bytes()\n'
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
	ret += '#[derive(Debug)]\n'
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
	ret += f'\tpub fn deserialize({name_lower}: {value_type}) -> Self {{\n'
	ret += f'\t\tSelf::new({name_lower})\n'
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
	ret += '#[derive(Debug)]\n'
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
	ret += f'\tpub fn deserialize(payload: [u8; {ast_model.size}]) -> Self {{\n'
	ret += f'\t\tSelf::new({value_type}::from_le_bytes(payload))\n'
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
