/// Partition the input array around a pivot element.
///
/// This function takes a mutable slice of integers and a pivot value.
/// It partitions the array into three parts:
/// - Numbers less than the pivot
/// - Numbers equal to the pivot
/// - Numbers greater than the pivot
///
/// The function returns a tuple containing the indices of the first and last
/// elements in the middle partition (numbers equal to the pivot).
///
/// # Arguments
///
/// * `input` - A mutable slice of integers to partition
/// * `pivot` - The pivot value around which to partition the array
///
/// # Returns
///
/// A tuple `(left, right)` where:
/// * `left` is the index of the first element equal to the pivot
/// * `right` is the index of the first element greater than the pivot
///
/// # Examples
///
/// ```
/// let mut numbers = [3, 1, 4, 1, 5, 9];
/// let pivot = 3;
/// let (left, right) = partition(&mut numbers, pivot);
/// // numbers[..left] contains elements < pivot
/// // numbers[left..right] contains elements == pivot
/// // numbers[right..] contains elements > pivot
/// ```
fn partition(input:&mut [i32], pivot: i32) -> (usize,usize) {

    // move numbers lower than pivot to the left
    let mut left = 0;
    for i in 0..input.len() {
        if input[i] < pivot {
            input.swap(i,left);
            left += 1;
        }
    }
    // now input[..left] are all numbers lower than pivot

    // move numbers greater than pivot to the right
    let mut right = input.len();
    for i in (left..input.len()).rev() {
        if input[i] > pivot {
            right -= 1;
            input.swap(i,right);
        }
    }
    // input[right..]: numbers greater than pivot

    // left is the index of the pivot and
    // right is the index of the first number greater than pivot
    (left,right)
}

use rand::Rng;

/// Sorts a slice of integers using the quicksort algorithm.
///
/// This function implements the quicksort algorithm using a divide and conquer approach.
/// It sorts the input slice in place, using a random pivot element for partitioning.
///
/// # Arguments
///
/// * `input` - A mutable slice of integers to be sorted
///
/// # Examples
///
/// ```
/// let mut numbers = [3, 1, 4, 1, 5, 9];
/// quicksort(&mut numbers);
/// assert_eq!(numbers, [1, 1, 3, 4, 5, 9]);
/// ```
///
fn quicksort(input:&mut [i32]) {
    if input.len() >= 2 {    

        // pivot = random element from the input
        let pivot = input[rand::rng().random_range(0..input.len())];

        // partition the input array around the pivot
        let (left,right) = partition(input, pivot);
        println!("\nL {} R {} P {} P {}", left, right, pivot, input[left]);
        
        println!("Left side {:?}", &input[..left]);
        println!("Right side {:?}", &input[right..]);
        
        quicksort(&mut input[..left]);
        quicksort(&mut input[right..]);
    }
}

fn main() {
    let mut q = vec![145,12,3,7,83,12,8,64];

    println!("Before {:?}", q);
    quicksort(&mut q);
    println!("After {:?}", q);
}
