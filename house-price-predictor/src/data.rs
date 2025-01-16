use polars::prelude::*;
use rand::thread_rng;
use rand::seq::SliceRandom;
use xgboost::{DMatrix, parameters, Booster};
// use xgboost::parameters::learning::Objective;
use aws_config::meta::region::RegionProviderChain;
use aws_sdk_s3::{Client, Error};

/// Downloads the external CSV file to disk
pub fn download_csv_file() -> anyhow::Result<String> {

    let url = "https://raw.githubusercontent.com/selva86/datasets/master/BostonHousing.csv";
    
    // Get the response from the URL
    let response = reqwest::blocking::get(url)?;
    
    // Get the bytes from the response into memory
    let bytes = response.bytes()?;

    let file_path = "boston_housing.csv";

    // Copy these bytes to a file on disk
    std::fs::write(file_path, bytes)?;

    Ok(file_path.to_string())
}

/// Loads a CSV file from disk into a Polars DataFrame
pub fn load_csv_file(file_path: &str) -> anyhow::Result<DataFrame> {

    let df = CsvReader::from_path(file_path)?
        .finish()?;

    println!("Loaded {} rows and {} columns", df.height(), df.width());
    println!("{:?}", df.head(Some(5)));
    
    Ok(df)
}

/// Randomly splits the data into training and testing sets
pub fn train_test_split(
    df: &DataFrame,
    test_size_perc: f64
) -> anyhow::Result<(DataFrame, DataFrame)> {

    // Generate a vector from 1 to the number of rows in the DataFrame
    let mut indices: Vec<usize> = (0..df.height()).collect();

    // Create a random number generator
    let mut rng = thread_rng();

    // Shuffle the indices in place
    indices.shuffle(&mut rng);

    // Split the indices into training and testing sets
    let split_idx = (df.height() as f64 * (1.0 - test_size_perc)).ceil() as usize;

    // Create the training and testing sets
    let train_indices = indices[0..split_idx].to_vec();
    let test_indices = indices[split_idx..].to_vec();

    // Convert from Vec<usize> to ChunkedArray<Int32Type>
    // We do this transformation because the DataFrame::take method
    // expects a ChunkedArray<Int32Type> as an argument.
    let train_indices_ca = UInt32Chunked::from_vec(
        "", train_indices.iter().map(|&x| x as u32).collect());
    let test_indices_ca = UInt32Chunked::from_vec(
        "", test_indices.iter().map(|&x| x as u32).collect());

    // Split the df DataFrame into training and testing sets
    // using the DataFrame::take method.
    let train_df = df.take(&train_indices_ca)?;
    let test_df = df.take(&test_indices_ca)?;
    
    println!("Training set size: {}", train_df.height());
    println!("Testing set size: {}", test_df.height());

    Ok((train_df, test_df))
}

/// Splits the given DataFrame into 2 dataframes: one for features and the other for the
/// target
pub fn split_features_and_target(df: &DataFrame) -> anyhow::Result<(DataFrame, DataFrame)> {

    let feature_names = vec![
        "crim", "zn", "indus", "chas", "nox", "rm", "age", "dis", "rad", "tax",
        "ptratio", "b", "lstat"
    ];
    let target_name = vec!["medv"];

    let features = df.select(feature_names)?;
    let target = df.select(target_name)?;

    Ok((features, target))
}