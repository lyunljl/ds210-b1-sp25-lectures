use ndarray::array;
use ndarray_linalg::*;

fn main() {
    // Create a 2D square matrix (3x3)
    let matrix = array![
        [1.0, 2.0, 3.0],
        [0.0, 1.0, 4.0],
        [5.0, 6.0, 0.0]
    ];

    // Compute the eigenvalues and eigenvectors
    match matrix.eig() {
        Ok((eigenvalues, eigenvectors)) => {
            println!("Eigenvalues:\n{:?}", eigenvalues);
            println!("Eigenvectors:\n{:?}", eigenvectors);
        },
        Err(e) => println!("Failed to compute eigenvalues and eigenvectors: {:?}", e),
    }
}
