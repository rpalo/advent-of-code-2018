"""Day 12: Subterranean Sustainability

Conway's game of life with plants!  See which plants will live or die
each generation.
"""

from collections import deque
from itertools import islice
from typing import List, Dict


class Plants(deque):
    """A row of plants centered around zero"""

    def __init__(self, *args):
        deque.__init__(self, *args)
        self.zero = 0

    def score(self) -> int:
        """Sums up all the indices (including negative) that have a plant"""
        return sum(i - self.zero for i, pot in enumerate(self) if pot == '#')

    def __getitem__(self, key) -> islice:
        """Allows slicing a deque"""
        if isinstance(key, slice):
            return islice(self, key.start, key.stop, key.step)
        else:
            return deque.__getitem__(self, key)

    def copy(self):
        result = Plants(self)
        result.zero = self.zero
        return result


def next_generation(plants: Plants, rules: Dict[str, str]) -> Plants:
    """Given a row of pots with/without plants and some rules, creates
    the next generation of plants.

    The only rules that could increase the length in either direction
    require 3 dots on the end to check, so this makes sure there are 3
    dots on each end just in case.
    """
    this_generation = plants.copy()
    if any(c == '#' for c in this_generation[:3]):
        this_generation.extendleft(['.']*3)
        this_generation.zero += 3
    if any(c == '#' for c in this_generation[len(this_generation) - 3:]):
        this_generation.extend(['.']*3)

    next_generation = this_generation.copy()
    for i in range(2, len(this_generation) - 2):
        next_generation[i] = rules.get("".join(this_generation[i-2:i+3]), '.')
    return next_generation


def parse_rules(text: str) -> Dict[str, str]:
    """Parses the conversion rules from a block of text"""
    rules = {}
    for line in text.splitlines():
        pattern, _, output = line.partition(" => ")
        rules[pattern] = output
    return rules


def age(plants: Plants, generations: int, rules: Dict[str, str]) -> Plants:
    """Ages a set of plants n generations"""
    for i in range(generations):
        plants = next_generation(plants, rules)
    return plants


if __name__ == "__main__":
    with open("python/data/day12.txt", "r") as f:
        rules = parse_rules(f.read())
    initial_plants = Plants(
        "##..#..##....#..#..#..##.#.###.######..#..###.#.#..##.###.#.##..###..#.#..#.##.##..###.#.#...#.##..")

    # Part 1
    plants = age(initial_plants, 20, rules)
    print(plants.score())

    # Part 2: Wherein it's #@%!# linear above about 300

    plants = age(initial_plants, 300, rules)
    print(plants.score() + (50000000000 - 300)*86)
