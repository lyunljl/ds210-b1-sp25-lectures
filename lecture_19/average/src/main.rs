fn divide_and_conquer_average(arr: &[f64]) -> (f64, usize) {
    // A divide and conquer approach to finding the average of an array using
    // recursion.
    //
    // This function takes a slice of floating point numbers and returns a tuple
    // containing the average and the number of elements in the array.
    //
    // Arguments:
    //     arr: A slice of floating point numbers
    //
    // Returns:
    //     A tuple containing the average and the number of elements in the array

    if arr.len() == 1 {
        return (arr[0], 1);
    } else {
        let mid = arr.len() / 2;
        let (left_sum, left_count) = divide_and_conquer_average(&arr[..mid]);
        let (right_sum, right_count) = divide_and_conquer_average(&arr[mid..]);
        return (left_sum + right_sum, left_count + right_count)
    }
}

fn main() {
    let arr = [1.0, 28.0, 32.0, 41.0, 25.0];
    let (total, num) = divide_and_conquer_average(&arr);
    println!("The average is {}", total/num as f64);

    let straightforward_avg = arr.iter().sum::<f64>() / arr.len() as f64;
    println!("The straightforward average is {}", straightforward_avg);
}
