"""Day 16: Chronal Classification

Identify and classify time machine opcodes
"""

from collections import defaultdict


def addr(state, inreg1, inreg2, outreg):
    """Adds the value of Reg 1 to the value of Reg 2 and stores in outreg"""
    result = state[::]
    result[outreg] = result[inreg1] + result[inreg2]
    return result


def addi(state, inreg, inval, outreg):
    """Adds the value of inreg to inval and stores it in outreg"""
    result = state[::]
    result[outreg] = result[inreg] + inval
    return result


def mulr(state, inreg1, inreg2, outreg):
    """Multiplies registers"""
    result = state[::]
    result[outreg] = result[inreg1] * result[inreg2]
    return result


def muli(state, inreg, inval, outreg):
    """Mulitplies immediate"""
    result = state[::]
    result[outreg] = result[inreg] * inval
    return result


def banr(state, inreg1, inreg2, outreg):
    """Bitwise AND registers"""
    result = state[::]
    result[outreg] = result[inreg1] & result[inreg2]
    return result


def bani(state, inreg, inval, outreg):
    """Bitwise AND immediate"""
    result = state[::]
    result[outreg] = result[inreg] & inval
    return result


def borr(state, inreg1, inreg2, outreg):
    """Bitwise OR registers"""
    result = state[::]
    result[outreg] = result[inreg1] | result[inreg2]
    return result


def bori(state, inreg, inval, outreg):
    """bitwise OR immediate"""
    result = state[::]
    result[outreg] = result[inreg] | inval
    return result


def setr(state, inreg, _, outreg):
    """Copies inreg to outreg"""
    result = state[::]
    result[outreg] = result[inreg]
    return result


def seti(state, inval, _, outreg):
    """Sets outreg to inval"""
    result = state[::]
    result[outreg] = inval
    return result


def gtir(state, inval, inreg, outreg):
    """If inval > register inreg, set outreg to 1 else 0"""
    result = state[::]
    result[outreg] = 1 if inval > result[inreg] else 0
    return result


def gtri(state, inreg, inval, outreg):
    """If register inreg > inval, set outreg to 1 else 0"""
    result = state[::]
    result[outreg] = 1 if result[inreg] > inval else 0
    return result


def gtrr(state, inreg1, inreg2, outreg):
    """If register inreg1 is greater than register inreg2, set outreg to 1 else 0"""
    result = state[::]
    result[outreg] = 1 if result[inreg1] > result[inreg2] else 0
    return result


def eqir(state, inval, inreg, outreg):
    """Immediate/register equality"""
    result = state[::]
    result[outreg] = 1 if inval == result[inreg] else 0
    return result


def eqri(state, inreg, inval, outreg):
    """Register/immediate equality"""
    result = state[::]
    result[outreg] = 1 if result[inreg] == inval else 0
    return result


def eqrr(state, inreg1, inreg2, outreg):
    """Register/register equality"""
    result = state[::]
    result[outreg] = 1 if result[inreg1] == result[inreg2] else 0
    return result


OPERATIONS = [addr, addi, mulr, muli, banr, bani, borr, bori,
              setr, seti, gtir, gtri, gtrr, eqir, eqri, eqrr]


def process_instructions(text):
    """Processes text into test cases"""
    cases = []
    for triplet in text.split("\n\n"):
        before, opcode, after = triplet.splitlines()
        before_registers = [int(c) for c in before if c.isdigit()]
        opcode = [int(num) for num in opcode.split()]
        after_registers = [int(c) for c in after if c.isdigit()]
        cases.append({
            "before": before_registers,
            "opcode": opcode,
            "after": after_registers,
        })
    return cases


def at_least_n_possible_opcodes(cases, n):
    """Returns the number of cases that could have applied at least n operations"""
    result = 0
    for case in cases:
        _, a, b, c = case["opcode"]
        possible_ops = sum(
            1 for op in OPERATIONS if op(case["before"], a, b, c) == case["after"]
        )
        if possible_ops >= n:
            result += 1
    return result


def decipher_opcodes(cases):
    """Given a set of operation input/outputs, identifies which opcodes
    go with which operations.
    """
    potential_mapping = defaultdict(set)
    available_op_ids = set()
    for case in cases:
        op_id, a, b, c = case["opcode"]
        available_op_ids.add(op_id)
        for op in OPERATIONS:
            if op(case["before"], a, b, c) == case["after"]:
                potential_mapping[op].add(op_id)

    result = {}
    while potential_mapping:
        for op, codes in potential_mapping.items():
            if len(codes) == 1:
                code = list(codes)[0]
                result[code] = op
                available_op_ids.remove(code)
            potential_mapping[op] = codes & available_op_ids
        potential_mapping = {op: codes for op,
                             codes in potential_mapping.items() if len(codes) > 0}
    return result


def result_of_instructions(instructions, optable):
    """Given a starting bank of registers with value 0, what are the contents
    of the registers after all instructions are processed"""
    registers = [0] * 4
    for instruction in instructions:
        opcode, a, b, c = instruction
        registers = optable[opcode](registers, a, b, c)
    return registers


if __name__ == "__main__":
    with open("python/data/day16.txt", "r") as f:
        text = f.read()

    # Part 1
    cases = process_instructions(text)
    print(at_least_n_possible_opcodes(cases, 3))

    # Part 2
    optable = decipher_opcodes(cases)
    with open("python/data/day16_ops.txt", "r") as f:
        instruction_text = f.read()
    instructions = [
        [int(value) for value in instruction.split()]
        for instruction in instruction_text.splitlines()
    ]
    print(result_of_instructions(instructions, optable))
