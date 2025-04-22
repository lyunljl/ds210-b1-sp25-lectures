use ndarray::Array2;
use ndarray::array;
use linfa_trees::DecisionTree;
use linfa::prelude::*;
// This lets us write `#[derive(Deserialize)]`.
use serde::Deserialize;
use std::fs::File;
use std::io::Write;

// We don't need to derive `Debug` (which doesn't require Serde), but it's a
// good habit to do it for all your types.
//
// Notice that the field names in this struct are NOT in the same order as
// the fields in the CSV data!
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct SerRecord {
    name: String,
    number: usize,
    year_born: usize,
    total_points: usize,
    PPG: f64,
}

fn process_csv_file() -> Vec<SerRecord> {
  let mut rdr = csv::Reader::from_path("players.csv").unwrap();
  let mut v:Vec<SerRecord> = Vec::new();
  // Loop over each record.
  for result in rdr.deserialize() {
    // An error may occur, so abort the program in an unfriendly way.
    // We will make this more friendly later!
    let record:SerRecord = result.expect("a CSV record");
    v.push(record);
  }
  return v;
}


fn main() {
  let mut v = process_csv_file();
  let mut flat_values: Vec<f64> = Vec::new();
  for s in &v {
    flat_values.push(s.total_points as f64);
    flat_values.push(s.PPG);
    flat_values.push(s.year_born as f64);
  }
  let array = Array2::from_shape_vec((v.len(), 3), flat_values).expect("Error creating ndarray");

  let likes_pizza = array![1,0,0,1,0];

  let dataset = Dataset::new(array, likes_pizza).with_feature_names(vec!["total points", "PPG", "year born"]);
  let decision_tree = DecisionTree::params()
        .max_depth(Some(3))
        .fit(&dataset)
        .unwrap();

  let accuracy = decision_tree.predict(&dataset).confusion_matrix(&dataset).unwrap().accuracy();
    
  println!("The accuracy is: {:?}", accuracy);

  let mut tikz = File::create("decision_tree_example.tex").unwrap();
    tikz.write_all(
        decision_tree
            .export_to_tikz()
            .with_legend()
            .to_string()
            .as_bytes(),
    )
    .unwrap();
    println!(" => generate tree description with `latex decision_tree_example.tex`!");
}
