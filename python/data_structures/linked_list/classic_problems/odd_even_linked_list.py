### https://medium.com/@saurav.agg19/odd-even-linked-list-a1cce50e737b
# https://leetcode.com/explore/learn/card/linked-list/219/classic-problems/1208/
# I look at this problem all morning, and didn't realize it is about the indices

from typing import Optional
# Definition for singly-linked list.
class ListNode:
    def __init__(self, val=0, next=None):
        self.val = val
        self.next = next
class Solution:
    def oddEvenList(self, head: Optional[ListNode]) -> Optional[ListNode]:
        # Every problem reference to linked list we need to check this
        if not head:
            return head
        if not head.next:
            return head
        
        # We can make two pointer, one for odd, one for even, due to everything start
        # from 1, we may have:

        oddHead=head
        evenStart=head.next

        # We should define the pointer for evenHead
        evenHead=head.next

        # We will create the odd list and even list, finally we set the last node of odd list
        # as even list head node

        # What's the condition of the loop?
        # My answer is we ues head and head.next, so we need to keep head.next cannot be None
        # evenHead=head.next
        while evenHead:
            # 1,2,3, as we know oddHead is 1, and we need to point 3 to oddHead's next
            # e.g:1->2->3, we let 1 point to 3=(evenHead.next)
            oddHead.next=evenHead.next

            if oddHead.next:
                oddHead=oddHead.next
            # Same as evenHead
            evenHead.next=oddHead.next
            if evenHead.next:
                evenHead=evenHead.next
        # As we know evenStart equals to evenHead, we add even to odd to finish this problems
        oddHead.next=evenStart
        # As we know oddHead is head at line No.22
        return head

# Time complexity O(n)
# Space complexity O(1)

