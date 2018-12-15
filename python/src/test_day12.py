"""Test cases for day 12"""

from .day12 import Plants, age, parse_rules


def test_part_one():
    rules = parse_rules("""...## => #
..#.. => #
.#... => #
.#.#. => #
.#.## => #
.##.. => #
.#### => #
#.#.# => #
#.### => #
##.#. => #
##.## => #
###.. => #
###.# => #
####. => #""")
    initial_plants = Plants("#..#.#..##......###...###")
    plants = age(initial_plants, 20, rules)
    assert 325 == plants.score()
