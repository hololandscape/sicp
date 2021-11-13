from typing import Any

class Node:
    def __init__(self,data) -> None:
        self.data=data
        self.next=None
    
    def __repr__(self) -> str:
        return f"Node({self.data})"
    
class LinkedList:
    # We use head field to describe the linker list
    def __init__(self) -> None:
        self.head=None
    
    ### 
    # Let's think what the operations we need to implement for a data structure, rather than only thingking in Linker list
    # We need to get the element by the index, get
    # We need to add the element to it, but the specify situation in linked list we have to consider the add to head or add to the any location we want to, 
    # as we know, in most cases, we will use the `head` node(the first) node to represent the whole list

    # Why we use this? Maybe it define in the paper(:^@) add(index) index=0 for head, index=len()-1 for the tail

    # Delete operation same as the add operation, there are two situations
    # 
    # Reverse, the confution problems
    # Empty, we can just check the (head node is None)
    ###

    # In order to easy for test
    def __repr__(self) -> str:
        return "->".join([str(item) for item in self])

    # As we said we need to add or delete the elements in linked list, so we need to find the element by iterator, we need to implement iterator
    # This can let us use enumerate(object)
    # https://docs.python.org/3.8/glossary.html#term-iterator
    def __iter__(self):
        # Here we use generator return the generator iterator of the node.data
        node=self.head
        while node:
            yield node.data
            node=node.next
    
    # we need to support len()
    def __len__(self):
        # we create a iterator as a parameter to tuple.__new__() to create a tuple with a fixed length, and return the length of this tuple
        return len(tuple(iter(self)))
    
    # we design build-in API for get function
    def __getitem__(self, index: int) -> Any:
        # We need to confirm index < len(self), due to index start from 0, len() from 1, we need keep index does not out of bound
        if not 0<=index<len(self):
            # return None or raise
            raise ValueError("index out of range")
        for i, node in enumerate(self):
            if i==index:
                return node
    # operational
    def __setitem__(self, index: int, data: Any)-> None:
        # We need to confirm index < len(self), due to index start from 0, len() from 1, we need keep index does not out of bound, this different to insert_nth
        if not 0<=index<len(self):
            raise ValueError("index out of range")
        # also this may work, but this will take O(n) n equal to the length of the linked list
        # for i, node in enumerate(self):
        #     if i ==index:
        #         node.data=data
        current=self.head
        # index =5, rang() doesn't includes 5, the last element ==4 and the sequences are from 0-4
        # and this will takes O(index), and we know index < n which n =len(self)
        for i in range(index):
            # i can be 0,1,2,3,4
            current=current.next
        current.data=data
    

    def insert_tail(self, data: Any)->None:
        self.insert_nth(len(self),data)
    def insert_head(self,data:Any)->None:
        self.insert_nth(0, data)
    def insert_nth(self,index: int,data:Any)-> None:
        # insert operations allow the index equals the length of the linked list, 
        # and if this,the node will be appended to the end of the linked list.
        # https://leetcode.com/explore/learn/card/linked-list/209/singly-linked-list/1290/

        # So, we must keep here, index<=len(self), and range(index-1) not range (index) below
        if not 0<=index <= len(self):
            raise IndexError("out of range")
        # O(1) due to we know the first node is head
        new_node=Node(data)
        if self.head==None:
            self.head=new_node
        elif index==0:
            new_node.next=self.head
            self.head=new_node
        # O(index-1) due to range does not includes the latest one
        else:
            temp=self.head
            """
            If we want to insert(3,5), which means add the data=5 before where the index=3
            """
            for _ in range(index-1):
                temp=temp.next
            new_node.next=temp.next
            temp.next=new_node

    def delete_tail(self)-> Any:
        return self.delete_nth(len(self)-1)
    def delete_head(self)-> Any:
        return self.delete_nth(0)
    def delete_nth(self, index: int)-> Any:
        if not 0<=index<len(self):
            raise IndexError("out of range.")
        delete_node=self.head
        if index==0:
            self.head=self.head.next
        else:
            temp=self.head
            for _ in range(index-1):
                temp=temp.next
            delete_node=temp.next
            temp.next=temp.next.next
        return delete_node.data
    
    def is_empty(self)->bool:
        return self.head is None
    def reverse(self)->None:
        prev=None
        current=self.head

        while current:
            # Store the current node's next node
            next_node=current.next
            # make current node's next point backwards
            current.next=prev
            # make the previous node be the current node
            prev=current
            # make the current node the next node(to progress iteration)
            current=next_node
        # return prev in order to put the head at the end
        self.head=prev
    def print_list(self)-> None:
        print(self)

def test_singly_linked_list() ->None:
    """
    >>> test_singly_linked_list()
    """
    linked_list=LinkedList()
    assert linked_list.is_empty() is True
    assert str(linked_list) == ""

    try:
        linked_list.delete_head()
        assert False # This should not happen
    except IndexError:
        assert True # This should happen
    
    try:
        linked_list.delete_tail()
        assert False # This should not happen
    except IndexError:
        assert True # This should happen
    
    for i in range(10):
        assert len(linked_list)==i
        linked_list.insert_nth(i,i+1)
    assert str(linked_list)=="->".join(str(i) for i in range(1,11))

    linked_list.insert_head(0)
    linked_list.insert_tail(11)
    assert str(linked_list)=="->".join(str(i) for i in range(0,12))

    assert linked_list.delete_head()==0
    assert linked_list.delete_nth(9)==10
    assert linked_list.delete_tail()==11
    assert len(linked_list)==9
    assert str(linked_list)=="->".join(str(i) for i in range(1,10))

    assert all(linked_list[i] ==i+1 for i in range(0,9)) is True

    for i in range(0,9):
        linked_list[i]=-i
    assert all(linked_list[i] ==-i for i in range(0,9)) is True

    linked_list.reverse()
    assert str(linked_list) == "->".join(str(i) for i in range(-8, 1))


def main():
    from doctest import testmod

    testmod()

    linked_list=LinkedList()
    linked_list.insert_head(1)
    linked_list.insert_head(2)
    print("\nPrint list:")
    linked_list.print_list()
    linked_list.insert_tail("3")
    linked_list.insert_tail("4")
    print("\nPrint list:")
    linked_list.print_list()
    print("\nDelete head")
    linked_list.delete_head()
    print("Delete tail")
    linked_list.delete_tail()
    print("\nPrint list:")
    linked_list.print_list()
    print("\nReverse linked list")
    linked_list.reverse()
    print("\nPrint list:")
    linked_list.print_list()
    print("\nString representation of linked list:")
    print(linked_list)
    print("\nReading/changing Node data using indexing:")
    print(f"Element at Position 1: {linked_list[1]}")
    linked_list[1] = 5
    print("New list:")
    print(linked_list)
    print(f"length of linked_list is : {len(linked_list)}")





if __name__ =="__main__":
    main()


    
