# Definition for singly-linked list.
from typing import Optional
class ListNode:
    def __init__(self, val=0, next=None):
        self.val = val
        self.next = next
class Solution:
    def mergeTwoLists(self, l1: Optional[ListNode], l2: Optional[ListNode]) -> Optional[ListNode]:
        if not l1:
            return l2
        if not l2:
            return l1
        node=ListNode()
        temp=node
        while True:
            if not l1:
                temp.next=l2
                break
            if not l2:
                temp.next=l1
                break
            if l1.val<=l2.val:
                temp.next=l1
                l1=l1.next
            else:
                temp.next=l2
                l2=l2.next
            temp=temp.next
        return node.next

    # With a bad runtime performance
    def recursiveMergeTwoLists(self, l1: Optional[ListNode], l2: Optional[ListNode]) -> Optional[ListNode]:
        if not l1:
            return l2
        if not l2:
            return l1
        temp=None
        if l1.val<=l2.val:
            temp=l1
            temp.next=self.recursiveMergeTwoLists(l1.next,l2)
        else:
            temp=l2
            temp.next=self.recursiveMergeTwoLists(l1,l2.next)
        return temp
            

# https://leetcode.com/explore/learn/card/linked-list/213/conclusion/1227/
# https://www.geeksforgeeks.org/merge-two-sorted-linked-lists/

# Time O(n)
# Space O(n)
