import typing


class Point:

    def __init__(self, x: int = 0, y: int = 0):
        self.x = x
        self.y = y

    def to_zero(self) -> int:
        return abs(self.x) + abs(self.y)

    def move_by(self, op: str):
        direction, step = op[0], int(op[1:])
        if direction == 'U':
            x, y = self.x, self.y + step
        elif direction == 'D':
            x, y = self.x, self.y - step
        elif direction == 'L':
            x, y = self.x - step, self.y
        elif direction == 'R':
            x, y = self.x + step, self.y
        else:
            raise ValueError(f'invalid direction of operation')
        return Point(x, y)


def draw_route(ops: typing.List[str]) -> typing.List[Point]:
    route = []
    point = Point()
    for op in ops:
        direction, step = op[0], int(op[1:])
        while step > 0:
            point = point.move_by(f'{direction}1')
            route.append(point)
            step -= 1
    return route


def get_indexed_route_joints(ops_a: typing.List[str], ops_b: typing.List[str]) -> typing.List[typing.Tuple[int, int, Point]]:
    route_a, route_b = draw_route(ops_a), draw_route(ops_b)
    joints_with_index = []
    count = len(route_a)
    for i, p_a in enumerate(route_a):
        for j, p_b in enumerate(route_b):
            if p_a.x == p_b.x and p_a.y == p_b.y:
                joints_with_index.append((i + 1, j + 1, p_a))
    return joints_with_index


class Solution:

    def manhattan_distance(self, ops_a: typing.List[str], ops_b: typing.List[str]) -> int:
        distances = []
        for _, _, point in get_indexed_route_joints(ops_a, ops_b):
            distances.append(point.to_zero())
        return min(distances)

    def fewest_combined_steps(self, ops_a: typing.List[str], ops_b: typing.List[str]) -> int:
        combined_steps = []
        for i, j, _ in get_indexed_route_joints(ops_a, ops_b):
            combined_steps.append(i + j)
        return min(combined_steps)


if __name__ == '__main__':
    solution = Solution()

    with open('input-part-1.txt') as f:
        input_part_1 = [x.strip().split(',') for x in f.readlines()]
    ops_a, ops_b = input_part_1
    answer_1 = solution.manhattan_distance(ops_a, ops_b)
    print(f'the answer to part-1 is {answer_1}')

    with open('input-part-2.txt') as f:
        input_part_2 = [x.strip().split(',') for x in f.readlines()]
    ops_a, ops_b = input_part_1
    answer_2 = solution.fewest_combined_steps(ops_a, ops_b)
    print(f'the answer to part-2 is {answer_2}')
