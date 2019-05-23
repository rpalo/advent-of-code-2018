"""Day 23: Experimental Emergency Teleportation

Find optimal locations for strong connections to nanobots in 3D space.
"""

from collections import Counter
from dataclasses import dataclass
from itertools import permutations
import re
from typing import List

@dataclass(frozen=True)
class Nanobot:
    """A nanobot in 3D space"""
    x: int
    y: int
    z: int
    r: int


def parse(text: str) -> List[Nanobot]:
    """Parses text where each line specifies a Nanobot."""
    bots = []
    pattern = re.compile(r"pos=<(-?\d+),(-?\d+),(-?\d+)>, r=(\d+)")
    for line in text.splitlines():
        match = pattern.match(line)
        if not match:
            raise ValueError(f"Invalid line: {line}")
        
        x, y, z, r = match.group(1, 2, 3, 4)
        bots.append(Nanobot(int(x), int(y), int(z), int(r)))

    return bots


def manhattan_distance(b1: Nanobot, b2: Nanobot) -> int:
    """The Manhattan Distance between two points is sum of steps required
    in X, Y, and Z (perpendicular movements only).
    """
    return abs(b1.x - b2.x) + abs(b1.y - b2.y) + abs(b1.z - b2.z)


def distance_from_radius_to_origin(bot: Nanobot) -> int:
    """Find the smallest distance from the surface of a sphere to the origin."""
    return max(bot.x + bot.y + bot.z - bot.r, 0)


def in_range_of_strongest(bots: List[Nanobot]) -> int:
    """Count how many bots are in range of the strongest (by radius) bot.  Include itself."""
    strongest = max(bots, key=lambda bot: bot.r)
    return sum(manhattan_distance(strongest, bot) <= strongest.r for bot in bots)


def best_point_distance(bots: List[Nanobot]) -> int:
    """Find the point in range of the most bots, breaking ties by closest to the origin.
    Return the manhattan distance from the origin to that point.
    """
    overlaps = []
    for b1 in bots:
        touching = frozenset(b2 for b2 in bots if b1.r + b2.r >= manhattan_distance(b1, b2))
        overlaps.append(touching)

    counts = Counter(a & b for a, b in permutations(overlaps, 2) if (a & b))
    biggest_group, _occurrences = counts.most_common(1)[0]

    return max(distance_from_radius_to_origin(bot) for bot in biggest_group)


if __name__ == "__main__":
    with open("data/day23.txt", "r") as f:
        text = f.read()
    bots = parse(text)
    print("Number in range of strongest:", in_range_of_strongest(bots))
    print("Distance from best point to origin:", best_point_distance(bots))
