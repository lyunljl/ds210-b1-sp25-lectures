# Section A1 Polls and Answers


## ------------ Midterm ----------------

## Modules in Rust
Mark all that is true about modules in Rust

- They can hide code from users
- They can limit access to some parts of the code
- They can be used to organize code
- They can contain public or private members
- They can contain other modules

F T T T T

## Libraries Binaries and Packages in Rust

Mark all that is true about libraries, binaries and packages in Rust

- A package can include many binaries
- A package can include a single binary
- A package can include many libraries
- A package can include a single library
- A package version can be modified after publication

T T F T F

## Networking Concepts

> Note: this wasn't covered in B1 and won't be on the final.

- IP is built on top of TCP
- DNS is build on top of TCP
- DNS is build on top of IP
- TCP is built on top of IP
- HTTP is build on top of TCP

F F T T T

## Linked Lists

- SLL add to front is O(1)
- DLL add to front is O(1)
- SLL add to back is O(1)
- DLL add to back is O(1)
- SLL and DLL add to middle are the same cost

T T F T T

## Stacks and Queues

- Stacks are FIFO
- Stacks are LIFO
- Queues are FIFO
- Queues are FILO
- Stacks are FILO

F T T F T

## BFS and DFS

- BFS is good for computing distances
- BFS is good for traversing trees
- DFS is good for computing distances
- DFS is good for traversing trees
- BFS is better than DFS

T F F T F

## Binary Heaps

- Push is Log(N)
- Pop is Log(N)
- Find minimum is Log(N)
- Using them to sort is Log(N)
- Find maximum in a min heap is Log(N)

T T F F F

## Shortest Paths

- A stack
- A queue
- A binary heap
- A priority queue
- A bipartite graph

F F T T F

## Lifetimes in Rust

- Each function input parameter can have its own lifetime
- Function output parameters must have lifetimes
- The default lifetime for a method's output is that of the self parameter
- The lifetime of a function's output if defined, must match one of its inputs
- Basic type inputs and outputs require lifetimes

T F T T F

## Closures vs Functions

- Closures do not accept arguments but functions do
- Closures can not return values but functions can
- Closures can be assigned to variables but functions can not
- Closures can be used with iterators but functions can not
- Closures can reference variables defined outside them while functions can not

F F F F T

## Binary Search and Rust

- Binary search works on sorted arrays, vectors, and slices
- Binary search is O(logN)
- Binary search works on a sorted vector of any type
- Binary search return None if it can't find the value
- You can use binary search to sort a vector

T T F F F

## Binary Trees

- In-order traversal first visits the root
- Pre-order traversal first visits the root
- Post-order traversals first visit the root
- All traversals cost O(logN)
- A tree must have O(logN) levels

F T F F F

## Binary Search Trees

- All nodes to the left are smaller than the parent node
- All nodes to the right are greater than the parent node
- All nodes to the left of the parent are smaller than all nodes to the right
- The depth of the tree is always logN
- An in-order traversal will show the nodes sorted by their values

T T T F T

## Greedy Algorithms

- They often work well in practice
- They are guaranteed to find an optimal solution
- They are always polynomial in complexity
- They prove that P=NP

T F T F

## Divide and Conquer Algorithms

- They are well suited for recursive solutions
- They divide the problem into smaller subproblems
- They are always polynomial in complexity
- They prove that P=NP

T T T F

## Neural Networks and linear algebra

- Forward propagation uses matrix matrix multiplication (*)
- Forward propagation uses matrix vector multiplication
- Forward propagation uses vector vector multiplication
- Backward propagation uses matrix matrix multiplication
- Backward propagation uses matrix vector multiplication
- Backward propagation uses vector vector multiplication

F T T T T T

> (*) For single input you would indeed use matrix-vector multipllication
> but you can create a batch matrix of inputs in which case
> it would be matrix-matrix multiplication.

## Rust plotters

- They can do 2D or 3D plots
- They can output to files
- They are OS and machine independent
- Their Jupyter integration is machine independent
- They can draw lines, histograms and points

T T F T T

## Cleaning CSV Data

- Reading a csv record returns an Option Enum
- Reading a csv record returns a Result enum
- CSV files can contain comments
- CSV files can contain headers
- The names in deserialized records must match the CSV file headers

F T T T T

## Decision Trees

- Entropy
- Gini
- Noise
- Complexity
- Big O Notation

T T F F F

## Linear Regression

- They compute a function from R->R
- They minimize an error metric
- They guarantee that the generated function fits all data points
- They can generate exponential functions (*)
- They can be used with categorical data without conversion

T T F F F

> (*) Note that conceivable you can have generalized least
> squares with an exponentiated input and linear parameter
> so you can argue this is True in that case.

## Regression Errors

- Mean Square Error
- Mean Absolute Error
- Mean Positive Error
- Mean Negative Error
- Error Variance

T T F F F


## Parallel Programming

- It is a way to solve problems faster
- It applies to all problems
- There are limits to how well it can work
- It can use threads
- It must use threads

T F T T F 
