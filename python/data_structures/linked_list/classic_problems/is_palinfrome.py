def is_palindrome(head)-> bool:
    slow,fast,prev=head, head, None
    # Floyd's Cycle Detection Algorithm
    # Faster reaches the end of the list, the slow pointer must than be in the middle.
    while fast and fast.next:
        slow, fast=slow.next,fast.next.next
    # With the slow at the middle, reverse the back half of the list with the help
    # of another variable to contain a reference to the prev and three way swap.

    # reversal
    # import prev to save the prev reference
    # Here we can think like this(note: they execute at same time) to reverse a linked list,
    # in order to break the reverse cycle and avoid an endless loop, prev.next=None
      # prev is slow
      # 
      # slow.next is slow
      # 
      # slow is slow.next
    prev, slow, prev.next=slow, slow.next, None
    while slow:
        temp=slow.next
        slow.next, prev, slow=prev, slow, temp
    # As a result, we should remember this like a function, like the way how to reverse linked list

    # as a result we already known prev is a reversal linked list
    # comparison
    fast, slow=head, prev
    while slow:
        if fast.val!=slow.val:
            return False
        fast, slow=fast.next, slow.next
    return True

# Time O(n)
# Space O(1)
# https://dev.to/seanpgallivan/solution-palindrome-linked-list-5721


def is_palindrome_stack(head):
    if not head or head.next:
        return False
    
    fast, slow, theHead=head,head
    # Why fast? Due to fast is the first pointer point to the end
    while fast:
        fast=fast.next.next
        slow=slow.next
    
    # here we got the middle point of the linked list

    # add to the stack
    stack=[slow.val]
    while slow:
        slow=slow.next
        stack.append(slow.val)

    # comparsion
    while stack:
        if stack.pop()!=theHead.val:
            return False
        theHead=theHead.next
    return True

# Time O(n)
# Space O(n+n/2)
