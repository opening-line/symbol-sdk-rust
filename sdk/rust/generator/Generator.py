#!/usr/bin/python

# Since the Rust language does not support default arguments,
# the new(args) and default() functions are provided as constructors.
# new(args): requires an argument
# default(): no argument required, default value is set

from pathlib import Path
import catparser
import lark
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

	with open(output_directory / 'models.rs', 'w', encoding='utf8', newline='\n') as output_file:
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
	# common (i.e. prepare)
	import re

	struct_name = ast_model.name
	def update_field_type(field_type):
		if type(field_type) == lark.lexer.Token:
			pass
		elif type(field_type) == catparser.ast.FixedSizeInteger:
			field_type.short_name = field_type.short_name.replace("uint", "u").replace("int", "i")
		elif type(field_type) == catparser.ast.Array:
			element_type = field_type.element_type
			update_field_type(element_type)
			field_type.convert_to_vec = f"Vec<{element_type}>"
		elif type(field_type) == str:
			pass
		else:
			raise "unexpected"

	print("\n##", struct_name)
	my_fields = {}
	for f in ast_model.fields:
		name = f.name
		if name == "type":
			name = "type_"
			f.name = name
		print(name)
		my_fields[name] = {}

	for f in ast_model.fields:
		name = f.name
		disposition = f.disposition
		field_type = f.field_type
		my_field = my_fields[name]
		if "init_value" in my_field.keys():
			init_value = my_field["init_value"]
		else:
			init_value = f.value
		
		update_field_type(field_type)

		is_member = True # temporarily
		if name == "size":
			is_member = False

		if disposition == "const":
			is_member = False
			const_member = name.split("_")[1].lower()
			if const_member == "type":
				const_member = "type_"
			for f in ast_model.fields:
				if const_member == f.name:
					my_fields[f.name]["init_value"] = f"Self::{name}"
		

		if type(field_type) == lark.lexer.Token:
			if init_value is None:
				init_value = f"{field_type}::default()"
			elif 'equals' in str(f.value) and 'if' in str(f.value):
				init_value = f"{field_type}::default()"
			elif "init_value" in my_field.keys():
				pass
			else:
				init_value = f"{field_type}::{init_value}"
			for_size_method = f"self.{name}.size()"
			for_deserialize_method = []
			for_serialize_method = []
			for_deserialize_method.append(f"let {name};")
			for_deserialize_method.append(f"({name}, payload) = {field_type}::deserialize(payload).unwrap();")
			for_serialize_method.append(f"serialized.append(&mut self.{name}.serialize());")

		elif type(field_type) == catparser.ast.FixedSizeInteger:
			if init_value is None:
				init_value = 0
			size = field_type.size
			for_size_method = size
			for_deserialize_method = []
			for_serialize_method = []
			for_deserialize_method.append(f"let mut bytes = [0u8; {size}];")
			for_deserialize_method.append(f"bytes.copy_from_slice(payload);")
			for_deserialize_method.append(f"let {name} = {field_type}::from_le_bytes(bytes);")
			for_deserialize_method.append(f"payload = &payload[{size} as usize..];")
			for_serialize_method.append(f'serialized.append(&mut self.{name}.to_le_bytes().to_vec());')
		elif type(field_type) == catparser.ast.Array:
			init_value = f"Vec::new()"
			size = field_type.size
			if size != 0:
				for f in ast_model.fields:
					if f.name == size:
						my_fields[f.name]["is_member"] = False
						my_fields[f.name]["size_of"] = name
						break

			for_deserialize_method = []
			for_serialize_method = []
			for_deserialize_method.append(f"let mut {name} = Vec::new();")
			if size == "__FILL__":
				for_deserialize_method.append(f"while payload.len() > 0 {{")
			else:
				for_deserialize_method.append(f"for _ in 0..{size} {{")

			element_type = field_type.element_type

			for_serialize_method.append(f"serialized.append(&mut self.{name}.iter().fold(")
			for_serialize_method.append(f"\tVec::new(), |mut a, b| {{")
			if type(element_type) == str:
				# element_type must have size method
				for_size_method = f'self.{name}.iter().map(|x| x.size()).sum::<usize>()'
				for_deserialize_method.append(f"\tlet element;")
				for_deserialize_method.append(f"\t(element, payload) = {element_type}::deserialize(payload).unwrap();")
				for_serialize_method.append(f"\t\ta.append(&mut b.serialize());")
				
			elif type(element_type) == catparser.ast.FixedSizeInteger:
				element_size = element_type.size
				for_size_method = f"{element_size} * self.{name}.len()"
				for_deserialize_method.append(f"\tlet mut bytes = [0u8; {element_size}];")
				for_deserialize_method.append(f"\tbytes.copy_from_slice(payload);")
				for_deserialize_method.append(f"\tlet element = {element_type}::from_le_bytes(bytes);")
				for_deserialize_method.append(f"\tpayload = &payload[{size} as usize..];")
				for_serialize_method.append(f"a.append(&mut b.to_le_bytes().to_vec());")
			else:
				raise "unexpected"
			
			for_deserialize_method.append(f"\t{name}.push(element);")
			for_deserialize_method.append(f"}}")
			for_serialize_method.append(f"\t\ta")
			for_serialize_method.append(f"\t}}")
			for_serialize_method.append(f"));")

		else:
			raise "unexpected"
		
		my_field["disposition"] = disposition
		my_field["field_type"] = field_type
		my_field["init_value"] = init_value
		my_field["is_member"] = is_member
		my_field["for_size_method"] = for_size_method
		my_field["for_deserialize_method"] = for_deserialize_method
		my_field["for_serialize_method"] = for_serialize_method

	# for f in ast_model.fields:
	# 	name = f.name
	# 	my_field = my_fields[name]	
	# 	print()
	# 	print("name:         ", name)
	# 	print("disposition:  ", my_field["disposition"])
	# 	print("field_type:   ", my_field["field_type"])
	# 	print("init_value:   ", my_field["init_value"])
	# 	print("is_member:    ", my_field["is_member"])
	# 	print("for_size_method:    ", my_field["for_size_method"])
	# 	print("for_deserialize_method:    ", my_field["for_deserialize_method"])
	# 	print("for_serialize_method:    ", my_field["for_serialize_method"])


	# anotation
	ret = ""
	ret += '/// ast_model.display_type == DisplayType.STRUCT\n'

	# structure
	ret += '#[derive(Debug)]\n'
	ret += f'pub struct {ast_model.name} {{\n'
	for f in ast_model.fields:
		name = f.name
		my_field = my_fields[name]	
		disposition = my_field["disposition"]
		field_type = my_field['field_type']
		if disposition == 'const':
			continue
		if not my_field["is_member"]:
			continue
		if type(field_type) == catparser.ast.Array:
			ret += f'\tpub {name}: Vec<{field_type.element_type}>,\n'
		else:
			ret += f'\tpub {name}: {field_type},\n'
	ret += '}\n'

	# implement
	ret += 'impl ' + ast_model.name + ' {\n'
	for f in ast_model.fields:
		name = f.name
		my_field = my_fields[name]	
		disposition = my_field["disposition"]
		field_type = my_field['field_type']
		init_value = my_field["init_value"]
		if disposition != 'const':
			continue
		ret += f'\tconst {name}: {field_type} = {init_value};\n'
			
	## constructor
	ret += '\tpub fn new() -> Self {\n'
	ret += '\t\tSelf {\n'
	for f in ast_model.fields:
		name = f.name
		my_field = my_fields[name]	
		disposition = my_field["disposition"]
		field_type = my_field['field_type']
		init_value = my_field["init_value"]
		if disposition == 'const':
			continue
		if not my_field["is_member"]:
			continue
		ret += f'\t\t\t{name}: {init_value},\n'
	ret += '\t\t}\n'
	ret += '\t}\n'

	ret += '\tpub fn default() -> Self {\n'
	ret += '\t\tSelf::new()\n'
	ret += '\t}\n'

	## size
	ret += '\tpub fn size(&self) -> usize {\n'
	ret += '\t\tlet mut size = 0;\n'
	for f in ast_model.fields:
		name = f.name
		my_field = my_fields[name]	
		disposition = my_field["disposition"]
		for_size_method = my_field["for_size_method"]
		if disposition == 'const':
			continue
		ret += f'\t\tsize += {for_size_method};\n'
	ret += '\t\tsize\n'
	ret += '\t}\n'

	## deserialize
	ret += '\tpub fn deserialize(mut payload: &[u8]) -> Option<(Self, &[u8])> {\n'
	for f in ast_model.fields:
		name = f.name
		my_field = my_fields[name]	
		disposition = my_field["disposition"]
		for_deserialize_method = my_field["for_deserialize_method"]
		if disposition == 'const':
			continue
		for line in for_deserialize_method:
			ret += f"\t\t{line}\n"
		if name == "size":
			ret += f'\t\tif size as usize > payload.len() {{ return None; }}\n'
		if disposition == "reserved":
			ret += f"\t\tif {name} != 0 {{ return None; }}\n"

	ret += f"\t\tlet self_ = Self {{\n"
	for f in ast_model.fields:
		name = f.name
		my_field = my_fields[name]	
		disposition = my_field["disposition"]
		for_deserialize_method = my_field["for_deserialize_method"]
		if disposition == 'const':
			continue
		if not my_field["is_member"]:
			continue
		ret += f"\t\t\t{name}: {name},\n"
	ret += "\t\t};\n"
	ret += '\t\tSome((self_, payload))\n'
	ret += '\t}\n'

	## serialize
	ret += f'\tpub fn serialize(&self) -> Vec<u8> {{\n'
	ret += '\t\tlet mut serialized = Vec::new();\n'
	for f in ast_model.fields:
		name = f.name
		my_field = my_fields[name]	
		disposition = my_field["disposition"]
		field_type = my_field['field_type']
		init_value = my_field["init_value"]
		for_serialize_method = my_field["for_serialize_method"]
		if disposition == 'const':
			continue
		if name == "size":
			ret += f'\t\tserialized.append(&mut self.size().to_le_bytes().to_vec());\n'
			
			
		elif "size_of" in my_field.keys():
			size_of = my_field["size_of"]
			ret += f'\t\tserialized.append(&mut self.{size_of}.len().to_le_bytes().to_vec());\n'
		else:
			for line in for_serialize_method:
				ret += f"\t\t{line}\n"


	ret += "\t\tserialized\n"
	ret += '\t}\n'

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
	ret += f'\tpub fn deserialize(payload: &[u8]) -> Option<(Self, &[u8])> {{\n'
	ret += f'\t\tif payload.len() < {ast_model.size} {{ return None; }}\n'
	ret += f'\t\tlet mut bytes = [0u8; {ast_model.size}];\n'
	ret += f'\t\tbytes.copy_from_slice(payload);\n'
	ret += f'\t\tmatch {value_type}::from_le_bytes(bytes) {{\n'
	ret += ''.join(
		list(
			map(
				lambda e: f'\t\t\t{e.value} => Some(({ast_model.name}::{e.name}, &payload[{ast_model.size}..])),\n',
				ast_model.values,
			)
		)
	)
	ret += '\t\t\t_ => None,\n'
	ret += '\t\t}\n'
	ret += '\t}\n'

	## serialize
	ret += f'\tpub fn serialize(&self) -> Vec<u8> {{\n'
	ret += f'\t\t(self.clone() as {value_type}).to_le_bytes().to_vec()\n'
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
	ret += f'\tpub fn deserialize(payload: &[u8]) -> Option<(Self, &[u8])> {{\n'
	ret += f'\t\tif payload.len() < {ast_model.size} {{ return None; }}\n'
	ret += f'\t\tlet mut bytes = [0u8; {ast_model.size}];\n'
	ret += f'\t\tbytes.copy_from_slice(payload);\n'
	ret += f'\t\tSome((Self::new(bytes), &payload[..{ast_model.size}]))\n'
	ret += '\t}\n'

	## serialize
	ret += f'\tpub fn serialize(&self) -> Vec<u8> {{\n'
	ret += '\t\tself.bytes.to_vec()\n'
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
	if ast_model.is_unsigned:
		value_type = 'u' + str(value_bit_width)
	else:
		raise 'unexpected'

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
	ret += f'\tpub fn deserialize(payload: &[u8]) -> Option<(Self, &[u8])> {{\n'
	ret += f'\t\tif payload.len() < {ast_model.size} {{ return None; }}\n'
	ret += f'\t\tlet mut bytes = [0u8; {ast_model.size}];\n'
	ret += f'\t\tbytes.copy_from_slice(payload);\n'
	ret += f'\t\tlet value = {value_type}::from_le_bytes(bytes);\n'
	ret += f'\t\tSome((Self::new(value), &payload[..{ast_model.size}]))\n'
	ret += '\t}\n'

	## serialize
	ret += f'\tpub fn serialize(&self) -> Vec<u8> {{\n'
	ret += '\t\tself.value.to_le_bytes().to_vec()\n'
	ret += '\t}\n'

	## to_string
	ret += f'\tpub fn to_string(&self) -> String {{\n'
	ret += f'\t\tformat!("0x{{:0{ast_model.size * 2}x}}", self.value)\n'
	ret += '\t}\n'

	# end
	ret += '}\n\n'

	return ret
