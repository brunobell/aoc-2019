import unittest

from main import Solution


class TestMain(unittest.TestCase):

    @classmethod
    def setUpClass(cls):
        cls.solution = Solution()

    def test_fuel_part_1(self):
        self.assertEqual(self.solution.fuel_part_1(12), 2)
        self.assertEqual(self.solution.fuel_part_1(14), 2)
        self.assertEqual(self.solution.fuel_part_1(1969), 654)
        self.assertEqual(self.solution.fuel_part_1(100756), 33583)

    def test_fuel_part_2(self):
        self.assertEqual(self.solution.fuel_part_2(14), 2)
        self.assertEqual(self.solution.fuel_part_2(1969), 966)
        self.assertEqual(self.solution.fuel_part_2(100756), 50346)
