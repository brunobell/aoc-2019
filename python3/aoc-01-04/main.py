import typing


def get_repeat_info(n: int) -> (bool, typing.Dict):
    digit, is_asc, repeat, hashmap = '0', True, 0, dict()
    for c in str(n):
        if c in hashmap:
            hashmap[c] = hashmap[c] + 1
        else:
            hashmap[c] = 1

        if digit <= c:
            if digit == c:
                digit, is_asc, repeat = c, is_asc, repeat + 1
            else:
                digit, is_asc, repeat = c, is_asc, 1
        else:
            digit, is_asc, repeat = c, False, 1
    return is_asc, hashmap


class Solution:

    def is_password_part_1(self, n: int, min_: int, max_: int) -> bool:
        is_asc, hashmap = get_repeat_info(n)
        min_repeat = min([x for x in hashmap.values() if x > 1] or [1])
        max_repeat = hashmap[max(hashmap.keys())]
        if is_asc and max_repeat <= max_ and min_repeat >= min_:
            return True
        else:
            return False

    def is_password_part_2(self, n: int, repeat: int) -> bool:
        is_asc, hashmap = get_repeat_info(n)
        valid = len([x for x in hashmap.values() if x == repeat]) > 0
        if is_asc and valid:
            return True
        else:
            return False


if __name__ == '__main__':
    solution = Solution()
    start, end = 353096, 843212
    answer_1 = []
    for n in range(start, end + 1):
        if solution.is_password_part_1(n, 2, 6):
            answer_1.append(n)
    print(f"the answer to part-1 is {len(answer_1)}")

    answer_2 = []
    for n in range(start, end + 1):
        if solution.is_password_part_2(n, 2):
            answer_2.append(n)
    print(f"the answer to part-2 is {len(answer_2)}")
