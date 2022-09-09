"""
# Definition for a Node.
class Node:
    def __init__(self, val=None, children=None):
        self.val = val
        self.children = children
"""

class Solution:
    def postorder(self, root: 'Node') -> List[int]:
        if root is None:
            return []
        order = []
        for n in root.children:
            order.extend(self.postorder(n))
        order.append(root.val)
        return order
