from typing import Optional
# Definition for singly-linked list.
class ListNode:
    def __init__(self, val=0, next=None):
        self.val = val
        self.next = next

class Solution:
    def removeNthFromEnd(self, head: Optional[ListNode], n: int) -> Optional[ListNode]:
        slow,fast=head,head
        for i in range(0,n):
            if fast.next is None:
# If n is equal to the number of nodes, delete the head node
                if i==n-1:
                    head=head.next
                return head
            fast=fast.next
        
        while fast.next:
            fast=fast.next
            slow=slow.next
# remove the node which we find
        if slow.next is not None:
            slow.next=slow.next.next
# we need to return the linked list
        return head

# Time complexities O(n) n is the length of the list
# https://redquark.org/leetcode/0019-remove-nth-node/
