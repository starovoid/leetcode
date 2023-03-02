class Solution:
    def getTargetCopy(self, original, cloned, target):
        def find(n1, n2, target):
            if n1 is None:
                return None
            if n1 == target:
                return n2
            found = find(n1.left, n2.left, target)
            if found:
                return found
            return find(n1.right, n2.right, target)

        return find(original, cloned, target)
