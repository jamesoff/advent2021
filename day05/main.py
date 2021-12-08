import re
import sys


class Grid:
    def __init__(self, edge: int) -> None:
        self.data = [[0 for i in range(0, edge)] for j in range(0, edge)]

    def increment_line(
        self,
        start_x: int,
        start_y: int,
        end_x: int,
        end_y: int,
        allow_diagonal: bool = False,
    ) -> None:
        if start_x == end_x:
            if start_y > end_y:
                start_y, end_y = end_y, start_y
            for y in range(start_y, end_y + 1):
                self.data[y][start_x] += 1
        elif start_y == end_y:
            if start_x > end_x:
                start_x, end_x = end_x, start_x
            for x in range(start_x, end_x + 1):
                self.data[start_y][x] += 1
        else:
            if not allow_diagonal:
                raise ValueError(
                    (
                        "not a horizontal or vertical line: "
                        f"{start_x},{start_y} -> {end_x},{end_y}"
                    )
                )
            if start_x > end_x:
                x_step = -1
                end_x -= 1
            else:
                x_step = 1
                end_x += 1
            if start_y > end_y:
                y_step = -1
                end_y -= 1
            else:
                y_step = 1
                end_y += 1
            x_list = range(start_x, end_x, x_step)
            y_list = range(start_y, end_y, y_step)
            if len(x_list) != len(y_list):
                raise ValueError(
                    "not a 45 degree line: " f"{start_x},{start_y} -> {end_x},{end_y}"
                )
            coord_list = zip(x_list, y_list)
            for coord in coord_list:
                self.data[coord[1]][coord[0]] += 1

    def count_points(self, min_value: int) -> int:
        row_sums = [sum([1 for x in row if x >= min_value]) for row in self.data]
        grand_sum = sum(row_sums)
        return grand_sum

    def parse_lines(self, data: str, allow_diagonal: bool = False) -> None:
        for line_number, line in enumerate(data.split("\n")):
            matches = re.search(r"(\d+),(\d+) -> (\d+),(\d+)", line)
            if matches:
                try:
                    self.increment_line(
                        int(matches.group(1)),
                        int(matches.group(2)),
                        int(matches.group(3)),
                        int(matches.group(4)),
                        allow_diagonal,
                    )
                except ValueError as error:
                    print(f"{error} at line {line_number}")

    def print_grid(self):
        for row in self.data:
            for cell in row:
                sys.stdout.write("." if cell == 0 else str(cell))
            sys.stdout.write("\n")


def challenge1():
    g = Grid(1000)
    with open("data.txt", "r") as fh:
        g.parse_lines(fh.read())
    print(g.count_points(2))


def challenge2():
    g = Grid(1000)
    with open("data.txt", "r") as fh:
        g.parse_lines(fh.read(), True)
    print(g.count_points(2))


def main():
    challenge1()
    challenge2()


if __name__ == "__main__":
    main()
