class node:
    """
    A node in a singly linked list.
    """
    id = 0
    value = ""
    next = None

    def __init__(self, id, value):
        self.id = id
        self.value = value
        self.next = None

    def push(self, start):
        """
        Push a node to the beginning of the list.
        """
        self.next = start.next
        start.next = self
        
    def pop(start):
        """
        Pop a node from the beginning of the list.
        """
        top_node = start.next
        start.next = top_node.next
        top_node.next = None
        return top_node

    def __str__(self):
        """
        Return a string representation of the node.
        """
        if self.next == None:
            val =  "None"
        else:
            val = "Next Id " + str(self.next.id)
        return ("ID: " + str(self.id) + ", Value: " +  self.value + ", Next: " +  val)


if __name__ == "__main__":
    # Create a start node
    start = node(id=0, value="Start")
    print(start)

    # Create a new node and push it onto the start node
    s = node(id=1, value="First node")
    s.push(start)
    print("")
    print(start)
    print(start.next)
    
    s2 = node(id=2, value="Second node")
    s2.push(start)
    print("")
    print(start)
    print(start.next)
    print(start.next.next)


    