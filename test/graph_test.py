import unittest
from Graph import Graph 

class nxTest(unittest.TestCase):

    def setUp(self):
        self.graph = Graph()
        self.initializeSampleData()
    
    def initializeSampleData(self):
        self.graph.add_edge("A", "B", weight=4)
        self.graph.add_edge("B", "C", weight=2)
        self.graph.add_edge("C", "D", weight=7)
        self.graph.add_edge("E", "F", weight=1)
        self.graph.add_edge("F", "G", weight=3)

    def test_node_count(self):
        self.assertEqual(len(self.graph), 8)