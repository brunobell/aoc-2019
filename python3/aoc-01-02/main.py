import copy


class Solution:

    def int_code(self, integers: list) -> list:
        loops = len(integers) // 4
        if loops > 0:
            for i in range(loops):
                start = 4 * i
                op_code = integers[start]
                pos_a = integers[start + 1]
                pos_b = integers[start + 2]
                pos_c = integers[start + 3]
                if op_code == 1:
                    integers[pos_c] = integers[pos_a] + integers[pos_b]
                elif op_code == 2:
                    integers[pos_c] = integers[pos_a] * integers[pos_b]
                elif op_code == 99:
                    break
                else:
                    raise ValueError
        return integers


if __name__ == '__main__':
    solution = Solution()

    with open('input-part-1.txt') as f:
        input_part_1 = [int(x) for x in f.read().strip().split(',')]
    input_part_1[1] = 12
    input_part_1[2] = 2
    answer_1 = solution.int_code(input_part_1)
    print(f'the answer to part-1 is {answer_1[0]}')

    with open('input-part-2.txt') as f:
        input_part_2 = [int(x) for x in f.read().strip().split(',')]
    target = 19690720
    stop = False
    for pos_a in range(99):
        for pos_b in range(99):
            input_part_2_cloned = copy.copy(input_part_2)
            input_part_2_cloned[1] = pos_a
            input_part_2_cloned[2] = pos_b
            if solution.int_code(input_part_2_cloned)[0] == target:
                print(
                    f'noun is {pos_a}, verb is {pos_b}, answer to part-2 is {pos_a * 100 + pos_b}')
                stop = True
                break
        if stop:
            break
