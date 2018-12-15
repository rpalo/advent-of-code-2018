"""Test cases for day 14"""

from .day14 import Board


def test_part_one_9():
    board = Board(3, 7)
    board.generate_n_scores(19)
    assert "5158916779" == "".join(str(i) for i in board.get_scores(10, 10))


def test_part_one_5():
    board = Board(3, 7)
    board.generate_n_scores(15)
    assert "0124515891" == "".join(str(i) for i in board.get_scores(6, 10))


def test_part_one_18():
    board = Board(3, 7)
    board.generate_n_scores(28)
    assert "9251071085" == "".join(str(i) for i in board.get_scores(19, 10))


def test_part_one_2018():
    board = Board(3, 7)
    board.generate_n_scores(2028)
    assert "5941429882" == "".join(str(i) for i in board.get_scores(2019, 10))
