use linfa::prelude::*;
use linfa_trees::DecisionTree;
use ndarray_rand::rand::SeedableRng;
use rand::rngs::SmallRng;
//use linfa_trees::DecisionTreeParams;

fn main() -> Result<(), Box<dyn std::error::Error>> {

    // Load the Iris dataset
    let mut iris = linfa_datasets::iris();

    let mut rng = SmallRng::seed_from_u64(42);

    // Split the data into training and testing sets
    let (train, test) = iris.clone()
        .shuffle(&mut rng)
        .split_with_ratio(0.8);

    // Extract the features (X) and target (y) for training and testing sets
    let x_train = train.records();
    let y_train = train.targets();
    let x_test = test.records();
    let y_test = test.targets();

    // Print the shape of the training and testing sets
    println!("x_train shape: ({}, {})", x_train.nrows(), x_train.ncols());
    println!("y_train shape: ({})", y_train.len());
    println!("x_test shape: ({}, {})", x_test.nrows(), x_test.ncols());
    println!("y_test shape: ({})", y_test.len());

    // Train the model on the training data
    let model = DecisionTree::params()
        .max_depth(Some(3))
        .fit(&train)?;

    // Evaluate the model's accuracy on the training set
    let train_accuracy = model.predict(&train)
        .confusion_matrix(&train)?
        .accuracy();
    println!("Training accuracy: {:.2}%", train_accuracy * 100.0);

    // Evaluate the model's accuracy on the test set
    let test_accuracy = model.predict(&test)
        .confusion_matrix(&test)?
        .accuracy();
    println!("Test accuracy: {:.2}%", test_accuracy * 100.0);
    
    // Define two models with depths 3 and 2
    let dt_params1 = DecisionTree::params().max_depth(Some(3));
    let dt_params2 = DecisionTree::params().max_depth(Some(2));

    // Create a vector of models
    let models = vec![dt_params1, dt_params2];

    // Train and cross-validation using the models
    let scores = iris.cross_validate_single(
        5, 
        &models, 
        |prediction, truth|
            Ok(prediction.confusion_matrix(truth.to_owned())?.accuracy()))?;
    println!("Cross-validation scores: {:?}", scores);

    // Perform cross-validation using fold
    let scores: Vec<_> = iris.fold(5).into_iter().map(|(train, valid)| {
       let model = DecisionTree::params()
          .max_depth(Some(3))
          .fit(&train).unwrap();
       let accuracy = model.predict(&valid).confusion_matrix(&valid).unwrap().accuracy();
       accuracy
    }).collect();
    
    println!("Cross-validation scores general: {:?} {}", scores, scores.iter().sum::<f32>()/scores.len() as f32);

    Ok(())

}
