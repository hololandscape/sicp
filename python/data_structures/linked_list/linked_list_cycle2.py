# Definition for singly-linked list.
class ListNode:
    def __init__(self, x):
        self.val = x
        self.next = None

class Solution:
    def detectCycle(self, head: ListNode) -> ListNode:
        if not head:
            return head
        slow=fast=head
        while (slow and fast and fast.next):
            slow,fast=slow.next,fast.next.next
            if slow==fast:
# here

# above is the Floyd’s Cycle-Finding Algorithm: https://www.geeksforgeeks.org/detect-loop-in-a-linked-list/

# below we already know the linked list has a cycle, and we just need to add a new point in oder to length from head too the meet node

# And we set the slow=head and iterate to get the distance of the meet node
                slow=head
                while fast!=slow:
                    fast,slow=fast.next,slow.next
                return fast
        return None

# time complexity: O(n)
# space complexity: O(1)

                