# tests/test_module1.py

import unittest
from graph import oppadd, oppsubtract

class TestModule1(unittest.TestCase):

    def test_add(self):
        self.assertEqual(oppadd(1, 2), -1)
        self.assertEqual(oppadd(-1, 1), -2)
    
    def test_subtract(self):
        self.assertEqual(oppsubtract(2, 1), 3)
        self.assertEqual(oppsubtract(2, 3), 5)

if __name__ == "__main__":
    unittest.main()