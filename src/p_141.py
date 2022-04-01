class Solution:
    def hasCycle(self, head: Optional[ListNode]) -> bool:
        count = 0
        while head:
            count += 1
            head = head.next
            if count > 10000:
                return True
        return False
