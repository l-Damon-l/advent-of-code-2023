import unittest

from main import hash_algorithm, Day11


class TestClass(unittest.TestCase):
    def test_hash_algorithm_returns_correct_value_for_test_values(self):
        test_cases = [
            ("H", 200),
            ("rn=1", 30),
            ("cm-", 253),
            ("qp=3", 97),
            ("rn", 0),
            ("qp", 1),
            ("pc", 3)
        ]

        for test_string, expected in test_cases:
            with self.subTest(test_string=test_string, expected=expected):
                self.assertEqual(expected, hash_algorithm(test_string))

    def test_part1_example_is_correct(self):
        day11 = Day11(is_test=True)
        self.assertEqual(1320, day11.part1())

    def test_part2_example_is_correct(self):
        day11 = Day11(is_test=True)
        self.assertEqual(145, day11.part2())


if __name__ == '__main__':
    unittest.main()
