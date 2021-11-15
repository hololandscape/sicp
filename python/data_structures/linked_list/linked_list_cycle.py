from typing import Optional


# Definition for singly-linked list.
class ListNode:
    def __init__(self, x):
        self.val = x
        self.next = None

class Solution:
    def hasCycle(self, head: Optional[ListNode]) -> bool:
        if head is None:
            return False
#
# there is not no parameter pos, and pos is not the key of this problems
#         def iterFor(head: Optional[ListNode]):
#             while head:
#                 yield head.data
#                 head=head.next
#         def lenFor(head:Optional[ListNode] )-> int:
#             return len(tuple(iterFor(head)))
        
#         if not 0<=pos<lenfor(head):
#             return false
        slow,fast=head,head.next
    
# We take O(n) time here due to we iterator the whole list, and O(1) space here, due to we don't use extra space. 
        while(slow!=fast):
            if (fast==None or fast.next==None):
                return False
            else:
                slow,fast=slow.next,fast.next.next
        return True