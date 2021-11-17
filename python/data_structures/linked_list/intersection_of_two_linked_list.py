# Definition for singly-linked list.
class ListNode:
    def __init__(self, x):
        self.val = x
        self.next = None

# The native approach here would be to store each node reference in a data structure until we saw the
# same one twice, but that would take O(N) extra space.

# We need to find a way to align the two linked lists, And the easiest way to do that is to concatemate them in opposite orders, A+B and B+A.This way, the ends of the two original lists will align on the second half of each merged list.

# We just need to make sure to string head B onto a and vice versa if one(but not both) list ends.

class Solution:
    def getIntersectionNode(self, headA: ListNode, headB: ListNode) -> ListNode:
        a,b=headA,headB
        # if linked list intersection, a must equal to b
        # For me is too tricky,there are two lists A and B, len(A) <len(B) with 2 nodes. 
        # And while we iterate the merge list, A will faster then B to iterate from head B due to we merge the list.
        # So, A will travel more nodes than B and reach the intersection node at the same time
        while(a!=b):
            # here need to deal with the situation that if a finish iterator, we need to let a to link B
            # we let A+B and B+A, and iterate both of them to check the intersection of two linked list
            a = headB if a is None else a.next
            b = headA if b is None else b.next
        return a

# Time Complexity: O(m+n) m=len(a), n=len(b)
# Space Complexity: O(1)

### https://dev.to/seanpgallivan/solution-intersection-of-two-linked-lists-478e
### https://aaronice.gitbook.io/lintcode/linked_list/intersection-of-two-linked-lists