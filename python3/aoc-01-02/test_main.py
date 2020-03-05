import unittest

from main import Solution


class TestMain(unittest.TestCase):

    @classmethod
    def setUpClass(cls):
        cls.solution = Solution()

    def test_int_code(self):
        self.assertEqual(
            self.solution.int_code([1, 0, 0, 0, 99]),
            [2, 0, 0, 0, 99])
        self.assertEqual(
            self.solution.int_code([2, 3, 0, 3, 99]),
            [2, 3, 0, 6, 99])
        self.assertEqual(
            self.solution.int_code([2, 4, 4, 5, 99, 0]),
            [2, 4, 4, 5, 99, 9801])
        self.assertEqual(
            self.solution.int_code([1, 1, 1, 4, 99, 5, 6, 0, 99]),
            [30, 1, 1, 4, 2, 5, 6, 0, 99])
