#!/usr/bin/python

from pathlib import Path

from catparser.DisplayType import DisplayType
from catparser.generators.util import build_factory_map, extend_models

from .EnumTypeFormatter import EnumTypeFormatter
from .FactoryFormatter import FactoryClassFormatter, FactoryFormatter
from .PodTypeFormatter import PodTypeFormatter
from .printers import BuiltinPrinter, create_pod_printer
from .StructTypeFormatter import StructFormatter
from .TypeFormatter import TypeFormatter


def create_printer(descriptor, name, is_pod):
	return (create_pod_printer if is_pod else BuiltinPrinter)(descriptor, name)


def to_type_formatter_instance(ast_model):
	type_formatter_class = {
		DisplayType.STRUCT: StructFormatter,
		DisplayType.ENUM: EnumTypeFormatter,
		DisplayType.BYTE_ARRAY: PodTypeFormatter,
		DisplayType.INTEGER: PodTypeFormatter
	}[ast_model.display_type]

	return type_formatter_class(ast_model)


def generate_files(ast_models, output_directory: Path):
	factory_map = build_factory_map(ast_models)
	extend_models(ast_models, create_printer)

	output_directory.mkdir(exist_ok=True)

	with open(output_directory / '__init__.py', 'w', encoding='utf8', newline='\n') as output_file:
		output_file.write(
			'''#!/usr/bin/python
#
# Code generated by catbuffer python generator; DO NOT EDIT.

from __future__ import annotations

from binascii import hexlify
from enum import Enum, Flag
from typing import ByteString, List, TypeVar

from ..ArrayHelpers import ArrayHelpers
from ..BaseValue import BaseValue
from ..ByteArray import ByteArray

# string or bytes
StrBytes = TypeVar('StrBytes', str, bytes)

'''
		)
		for ast_model in ast_models:
			generator = TypeFormatter(to_type_formatter_instance(ast_model))
			output_file.write(str(generator))
			output_file.write('\n\n')

		factories = []
		for ast_model in ast_models:
			if DisplayType.STRUCT == ast_model.display_type and ast_model.is_abstract:
				factory_generator = FactoryClassFormatter(FactoryFormatter(factory_map, ast_model))
				factories.append(str(factory_generator))

		output_file.write('\n\n'.join(factories))


class Generator:
	@staticmethod
	def generate(ast_models, output):
		print(f'python catbuffer generator called with output: {output}')
		generate_files(ast_models, Path(output))
