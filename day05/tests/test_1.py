from main import Grid


def test_init():
    g = Grid(3)
    assert g.data == [[0, 0, 0], [0, 0, 0], [0, 0, 0]]


def test_x():
    g = Grid(3)
    g.increment_line(0, 0, 2, 0)
    assert g.data == [[1, 1, 1], [0, 0, 0], [0, 0, 0]]
    g.increment_line(0, 0, 1, 0)
    assert g.data == [[2, 2, 1], [0, 0, 0], [0, 0, 0]]


def test_y():
    g = Grid(3)
    g.increment_line(0, 0, 0, 2)
    assert g.data == [[1, 0, 0], [1, 0, 0], [1, 0, 0]]
    g.increment_line(0, 0, 0, 1)
    assert g.data == [[2, 0, 0], [2, 0, 0], [1, 0, 0]]


def test_x_inverse():
    g = Grid(3)
    g.increment_line(2, 0, 0, 0)
    assert g.data == [[1, 1, 1], [0, 0, 0], [0, 0, 0]]
    g.increment_line(1, 0, 0, 0)
    assert g.data == [[2, 2, 1], [0, 0, 0], [0, 0, 0]]


def test_y_inverse():
    g = Grid(3)
    g.increment_line(0, 2, 0, 0)
    assert g.data == [[1, 0, 0], [1, 0, 0], [1, 0, 0]]
    g.increment_line(0, 1, 0, 0)
    assert g.data == [[2, 0, 0], [2, 0, 0], [1, 0, 0]]


def test_example():
    g = Grid(10)
    g.parse_lines(
        """
        0,9 -> 5,9
        8,0 -> 0,8
        9,4 -> 3,4
        2,2 -> 2,1
        7,0 -> 7,4
        6,4 -> 2,0
        0,9 -> 2,9
        3,4 -> 1,4
        0,0 -> 8,8
        5,5 -> 8,2
                  """
    )
    print(g.data)
    assert g.count_points(2) == 5

def test_example_2():
    g = Grid(10)
    g.parse_lines(
        """
        0,9 -> 5,9
        8,0 -> 0,8
        9,4 -> 3,4
        2,2 -> 2,1
        7,0 -> 7,4
        6,4 -> 2,0
        0,9 -> 2,9
        3,4 -> 1,4
        0,0 -> 8,8
        5,5 -> 8,2
                  """, True
    )
    g.print_grid()
    assert g.count_points(2) == 12

