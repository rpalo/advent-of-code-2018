"""Test cases for day 13"""

from .day13 import Mine, Vector


def test_part_one():
    mine_map = r"""
/->-\        
|   |  /----\
| /-+--+-\  |
| | |  | v  |
\-+-/  \-+--/
  \------/   
""".strip()
    mine = Mine(mine_map)
    assert mine.find_first_collision() == Vector(7, 3)


def test_part_two():
    mine_map = r"""
/>-<\  
|   |  
| /<+-\
| | | v
\>+</ |
  |   ^
  \<->/
""".strip()
    mine = Mine(mine_map)
    assert mine.last_cart_location() == Vector(6, 4)
