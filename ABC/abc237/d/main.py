
class Cell:
    def __init__(self, val):
        self.val = val
        self.prev = None
        self.next = None

    def __repr__(self):
        return f"Cell({self.val} {self.prev.val} {self.next.val})"

class LinkedList:
    def __init__(self, head):
        self.head = head
    
    def insert_left(self, next, item):
        if self.head == next:
            self.head == item
        if next.prev:
            item.prev = next.prev
        next.prev = item
        item.next = next
    
    def insert_right(self, prev, item):
        if prev.next:
            item.next = prev.next
        prev.next = item
        item.prev = prev
    
    def traverse(self):
        now = self.head
        while now.val:
            print(now.val)
            now = now.next

if __name__ == "__main__":
    N = int(input())
    S = input()

    now = Cell(0)
    linked_list = LinkedList(now)

    for i, c in enumerate(S):
        item = Cell(i+1)
        if c == "L":
            linked_list.insert_left(now, item)
        else:
            linked_list.insert_right(now, item)
        now = item
        print(now)
    
    linked_list.traverse()
