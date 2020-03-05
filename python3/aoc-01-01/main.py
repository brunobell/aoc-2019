class Solution:

    def fuel_part_1(self, mass: int) -> int:
        return mass // 3 - 2

    def fuel_part_2(self, mass: int) -> int:
        fuel_list = []
        while True:
            mass = self.fuel_part_1(mass)
            if mass > 0:
                fuel_list.append(mass)
            else:
                break
        return sum(fuel_list)


if __name__ == '__main__':
    solution = Solution()

    with open('input-part-1.txt') as f:
        input_part_1 = f.readlines()
    answer_1 = sum(map(lambda line: solution.fuel_part_1(
        int(line.strip())), input_part_1))
    print(f'the answer to part-1 is {answer_1}')

    with open('input-part-2.txt') as f:
        input_part_2 = f.readlines()
    answer_2 = sum(map(lambda line: solution.fuel_part_2(
        int(line.strip())), input_part_2))
    print(f'the answer to part-2 is {answer_2}')
