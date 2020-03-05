import unittest

from main import Solution


class TestMain(unittest.TestCase):

    @classmethod
    def setUpClass(cls):
        cls.solution = Solution()

    def test_is_password_part_1(self):
        self.assertEqual(self.solution.is_password_part_1(111111, 2, 6), True)
        self.assertEqual(self.solution.is_password_part_1(223450, 2, 6), False)
        self.assertEqual(self.solution.is_password_part_1(123789, 2, 6), False)

    def test_is_password_part_2(self):
        self.assertEqual(self.solution.is_password_part_2(112233, 2), True)
        self.assertEqual(self.solution.is_password_part_2(123444, 2), False)
        self.assertEqual(self.solution.is_password_part_2(111122, 2), True)
