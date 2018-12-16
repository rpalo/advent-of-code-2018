"""Day 13: Mine Cart Madness

Figure out when carts on tracks will crash
"""

from typing import List


class Vector:
    """A 2D vector.  Right is +X, Down is +Y"""

    def __init__(self, x, y):
        self.x = x
        self.y = y

    def __add__(self, other):
        return Vector(self.x + other.x, self.y + other.y)

    def __eq__(self, other):
        return self.x == other.x and self.y == other.y

    def __repr__(self):
        return f"Vector({self.x}, {self.y})"

    def __str__(self):
        return f"<{self.x}, {self.y}>"

    def __hash__(self):
        return hash((self.x, self.y))


NORTH = Vector(0, -1)
SOUTH = Vector(0, 1)
EAST = Vector(1, 0)
WEST = Vector(-1, 0)
DIRECTIONS = [NORTH, EAST, SOUTH, WEST]


class Cart:
    """A cart that drives around the mine"""

    def __init__(self, position: Vector, direction: Vector):
        self.position = position
        self.direction = direction
        self.next_turn = "left"

    def move(self):
        """Updates position based on direction"""
        self.position += self.direction

    def turn_right(self):
        self.direction = DIRECTIONS[
            (DIRECTIONS.index(self.direction) + 1) % len(DIRECTIONS)
        ]

    def turn_left(self):
        self.direction = DIRECTIONS[
            (DIRECTIONS.index(self.direction) - 1) % len(DIRECTIONS)
        ]

    def handle_next_turn(self):
        """Update own direction based on the next turn value specified in the prompt"""
        if self.next_turn == "left":
            self.turn_left()
            self.next_turn = "straight"
        elif self.next_turn == "straight":
            self.next_turn = "right"
        elif self.next_turn == "right":
            self.turn_right()
            self.next_turn = "left"


class Mine:
    """A mine that has a bunch of different carts on tracks"""

    def __init__(self, map: str):
        self.map = []
        self.carts: List[Cart] = []
        for y, line in enumerate(map.splitlines()):
            self.map.append([])
            for x, c in enumerate(line):
                if c == '>':
                    self.carts.append(Cart(Vector(x, y), EAST))
                    self.map[-1].append('-')
                elif c == '<':
                    self.carts.append(Cart(Vector(x, y), WEST))
                    self.map[-1].append('-')
                elif c == '^':
                    self.carts.append(Cart(Vector(x, y), NORTH))
                    self.map[-1].append('|')
                elif c == 'v':
                    self.carts.append(Cart(Vector(x, y), SOUTH))
                    self.map[-1].append('|')
                else:
                    self.map[-1].append(c)

    def find_first_collision(self) -> Vector:
        """Simulates the mine until two carts collide.  Returns that x, y pair"""
        while True:
            self.carts.sort(key=lambda cart: (
                cart.position.y, cart.position.x))

            for cart in self.carts:
                cart.move()
                self.turn_cart(cart)

                if self.is_collision(cart):
                    return cart.position

    def last_cart_location(self) -> Vector:
        """Simulates the mine, removing carts when they collide.
        Returns the x, y pair of the last cart standing
        """
        while True:
            self.sort_carts()

            crashed = set()
            for cart in self.carts:
                if cart.position in crashed:
                    continue
                cart.move()
                self.turn_cart(cart)

                if self.is_collision(cart):
                    crashed.add(cart.position)
                    continue

            self.carts = [cart
                          for cart in self.carts
                          if cart.position not in crashed]
            if len(self.carts) == 1:
                return self.carts[0].position

    def sort_carts(self):
        """Carts are always run in reading order, top to bottom, left to right"""
        self.carts.sort(key=lambda cart: (cart.position.y, cart.position.x))

    def is_collision(self, cart: Cart) -> bool:
        """Checks whether or not a cart is running into any other carts"""
        return sum(other.position == cart.position for other in self.carts) > 1

    def turn_cart(self, cart: Cart):
        """Depending on what kind of track the cart is on, turn it appropriately"""
        track_type = self.map[cart.position.y][cart.position.x]
        if track_type == '+':
            cart.handle_next_turn()
        elif track_type == '/':
            if cart.direction == NORTH or cart.direction == SOUTH:
                cart.turn_right()
            else:
                cart.turn_left()
        elif track_type == '\\':
            if cart.direction == NORTH or cart.direction == SOUTH:
                cart.turn_left()
            else:
                cart.turn_right()


if __name__ == "__main__":
    with open("python/data/day13.txt", "r") as f:
        mine_map = f.read()

    # Part 1
    mine = Mine(mine_map)
    print(mine.find_first_collision())

    # Part 2
    mine = Mine(mine_map)
    print(mine.last_cart_location())
