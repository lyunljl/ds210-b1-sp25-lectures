fn merge(v1:Vec<i32>, v2:Vec<i32>) -> Vec<i32> {
    // Take two sorted vectors and merge them into a single sorted vector
    //
    // This function takes two vectors of integers and returns a new vector
    // containing all the elements from both vectors in sorted order.
    //
    // Arguments:
    //     v1: A sorted vector of integers
    //     v2: A sorted vector of integers
    //
    // Returns:
    //     A new vector containing all the elements from both vectors in sorted order

    let (l1,l2) = (v1.len(), v2.len());
    let mut merged = Vec::with_capacity(l1+l2); // preallocate memory
    let (mut i1, mut i2) = (0,0);
    
    while i1 < l1 {
        if (i2 == l2) || (v1[i1] <= v2[i2]) {
            merged.push(v1[i1]);
            i1 += 1;
        } else {
            merged.push(v2[i2]);
            i2 += 1;
        }
    }
    while i2 < l2 {
        merged.push(v2[i2]);
        i2 += 1;
    }
    merged
}

fn merge_sort(input:&[i32]) -> Vec<i32> {
    // Implement merge sort using divide and conquer
    //
    // This function takes a slice of integers and returns a new vector
    // containing all the elements from the input slice in sorted order.
    //
    // Arguments:
    //     input: A slice of integers
    //
    // Returns:
    //     A new vector containing all the elements from the input slice in sorted order

    if input.len() <= 1 {
        input.to_vec()
    } else {
        let split = input.len() / 2;
        let v1 = merge_sort(&input[..split]);
        let v2 = merge_sort(&input[split..]);
        merge(v1,v2)
    }
}

fn main() {
    let v = vec![2,4,21,6,2,32,62,0,-2,8];
    let sorted = merge_sort(&v);
    println!("Sorted vector: {:?}", sorted);
}
