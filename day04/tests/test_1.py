import re

from src.day04 import Board, load_boards


def test_board():
    b = Board([[1, 2, 3], [4, 5, 6], [7, 8, 9]])
    assert not b.winner()


data = """
22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7
"""
draws = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1"

def test_example_1():
    boards = load_boards(data)
    assert len(boards) == 3

    winner = None
    winning_boards = 0
    for turn, draw in enumerate(draws.split(",")):
        draw = int(draw)
        for board_num, board in enumerate(boards):
            if board.winner():
                continue
            board.draw(draw)
            if board.winner():
                print(f"board {board_num} wins after {turn} turns")
                winner = board
                winning_boards += 1
            if winning_boards == len(boards):
                break
        if winning_boards == len(boards):
            break
    assert winner == boards[2]
    assert winner.score() == 188
    assert winner.score() * draw == 4512

def test_challenge2():
    boards = load_boards(data)
    winner = None
    winning_boards = 0
    for turn, draw in enumerate(draws.split(",")):
        draw = int(draw)
        for board_num, board in enumerate(boards):
            if board.winner():
                continue
            board.draw(draw)
            if board.winner():
                print(f"Board {board_num} wins after {turn} turns")
                winner = board
                winning_boards += 1
            if winning_boards == len(boards):
                break
        if winning_boards == len(boards):
            break
    print(f"board score: {winner.score()}")
    print(f"challenge answer: {winner.score() * draw}")
    assert winner == boards[1]
    assert winner.score() == 148
    assert draw == 13
    assert winner.score() * draw == 1924
