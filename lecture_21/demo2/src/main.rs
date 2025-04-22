:dep csv = { version = "^1.3.1" }
:dep ndarray = { version = "^0.15.6" }
:dep ndarray-csv = { version = "^0.5.3" }

extern crate ndarray;
extern crate ndarray_csv;

use csv::{ReaderBuilder, WriterBuilder};
use ndarray::{array, Array2};
use ndarray_csv::{Array2Reader, Array2Writer};
use std::error::Error;
use std::fs::File;

fn main() -> Result<(), Box<dyn Error>> {
    // Our 2x3 test array
    let array = array![[1, 2, 3], [4, 5, 6]];

    // Write the array into the file.
    {
        let file = File::create("test.csv")?;
        let mut writer = WriterBuilder::new().has_headers(false).from_writer(file);
        writer.serialize_array2(&array)?;
    }

    // Read an array back from the same file
    let file = File::open("test.csv")?;
    let mut reader = ReaderBuilder::new().has_headers(false).from_reader(file);
    let array_read: Array2<i32> = reader.deserialize_array2((2, 3))?;

    // Ensure that we got the original array back
    assert_eq!(array_read, array);
    println!("{:?}", array_read);
    Ok(())
}

