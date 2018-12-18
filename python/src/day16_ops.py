"""Day 16: Bytecode operations"""


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
