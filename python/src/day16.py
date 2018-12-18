"""Day 16: Chronal Classification

Identify and classify time machine opcodes
"""

from collections import defaultdict, namedtuple
from itertools import chain

from day16_ops import OPERATIONS

Example = namedtuple("Example", ["before", "opcode", "after"])


def parse_examples(text):
    """Parses text to build 'before, command, after' examples.

    Input text has the pattern:
    Before: [0, 0, 0, 0]
    12 4 2 1
    After: [1, 2, 3, 4]
    """
    examples = []
    for triplet in text.split("\n\n"):
        before, command, after = triplet.splitlines()
        before_registers = [int(c) for c in before if c.isdigit()]
        opcode = [int(num) for num in command.split()]
        after_registers = [int(c) for c in after if c.isdigit()]
        examples.append(Example(
            before=before_registers,
            opcode=opcode,
            after=after_registers,
        ))
    return examples


def at_least_n_possible_opcodes(examples, n, operations):
    """Returns the number of cases that could have applied at least n operations"""
    result = 0
    for example in examples:
        _, a, b, c = example.opcode
        possible_ops = sum(
            1 for op in operations if op(example.before, a, b, c) == example.after
        )
        if possible_ops >= n:
            result += 1
    return result


def potential_op_ids(examples, operations):
    """Figure out which op_ids *could* match which operations"""
    potential_ids = defaultdict(set)
    for example in examples:
        op_id, a, b, c = example.opcode
        for op in operations:
            if op(example.before, a, b, c) == example.after:
                potential_ids[op].add(op_id)
    return potential_ids


def identify_opcodes(examples, operations):
    """Given a set of operation input/outputs, identifies which opcodes
    go with which operations.
    """
    potential_ids = potential_op_ids(examples, operations)
    available_op_ids = set(chain.from_iterable(potential_ids.values()))

    # Whittle down the possibilities by finding each op id that is guaranteed
    # to match to a particular operation function.  Then, remove that op id
    # from the running for every other op function.  This will cause more
    # op functions to only have one possible op id.  Lather, rinse, repeat.
    result = {}
    while potential_ids:
        for op, codes in potential_ids.items():
            if len(codes) == 1:
                code = list(codes)[0]
                result[code] = op
                available_op_ids.remove(code)
            # Remove any op_ids that are "spoken for" by another operation
            # Thank God for set mathematics
            potential_ids[op] = codes & available_op_ids

        # Filter out any operation functions that don't need figured out anymore
        potential_ids = {
            op: codes
            for op, codes in potential_ids.items()
            if len(codes) > 0
        }
    return result


def process_instructions(instructions, optable):
    """Given a starting bank of registers with value 0, what are the contents
    of the registers after all instructions are processed"""
    registers = [0] * 4
    for instruction in instructions:
        op_id, a, b, c = instruction
        registers = optable[op_id](registers, a, b, c)
    return registers


if __name__ == "__main__":
    with open("python/data/day16.txt", "r") as f:
        text = f.read()

    # Part 1
    examples = parse_examples(text)
    print(at_least_n_possible_opcodes(examples, 3, OPERATIONS))

    # Part 2
    optable = identify_opcodes(examples, OPERATIONS)
    with open("python/data/day16_ops.txt", "r") as f:
        instruction_text = f.read()
    instructions = [
        [int(value) for value in instruction.split()]
        for instruction in instruction_text.splitlines()
    ]
    print(process_instructions(instructions, optable))
