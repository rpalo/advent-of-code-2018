"""Day 14: Chocolate Charts

Figuring out scores for hot chocolate through iterative process.
"""

from typing import List


class Board:
    """Keeps track of hot chocolate recipe scores"""

    def __init__(self, elf1_score, elf2_score):
        self.scores = [elf1_score, elf2_score]
        self.elf1 = 0
        self.elf2 = 1

    def generate_new_recipes(self):
        """Generates one or two new recipes by combining the current ones"""
        new_score = self.scores[self.elf1] + self.scores[self.elf2]
        self.scores.extend(Board.digits(new_score))

    @staticmethod
    def digits(number: int) -> List[int]:
        """Given a number, returns a list of its digits"""
        return [int(digit) for digit in str(number)]

    def select_new_recipes(self):
        """Each elf selects a new recipe based on their current one"""
        self.elf1 = (self.elf1 + self.scores[self.elf1] + 1) % len(self.scores)
        self.elf2 = (self.elf2 + self.scores[self.elf2] + 1) % len(self.scores)

    def tick(self):
        """One iteration cycle of creating recipes"""
        self.generate_new_recipes()
        self.select_new_recipes()

    def generate_n_scores(self, n: int) -> List[int]:
        """Returns the next ten scores to be generated"""
        current_scores = len(self.scores)
        while len(self.scores) < current_scores + n:
            self.tick()

        return self.scores[current_scores:current_scores + n]

    def get_scores(self, start: int, count: int) -> List[int]:
        """Returns the scores on the board.  1-based counting"""
        return self.scores[start - 1:start - 1 + count]


if __name__ == "__main__":
    # Part 1
    board = Board(3, 7)
    board.generate_n_scores(440231 + 10)
    print("".join(str(i) for i in board.get_scores(440231 + 1, 10)))
