"""Test cases for day 16"""

from .day16 import process_instructions, at_least_n_possible_opcodes


def test_part_one():
    text = """Before: [3, 2, 1, 1]
9 2 1 2
After:  [3, 2, 2, 1]"""
    cases = process_instructions(text)
    assert 1 == at_least_n_possible_opcodes(cases, 3)
