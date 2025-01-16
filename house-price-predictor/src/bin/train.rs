use clap::Parser;

use house_price_predictor::data::{
    download_csv_file,
    load_csv_file,
    train_test_split,
    split_features_and_target
};
use house_price_predictor::model::train_xgboost_model;
use house_price_predictor::aws::push_model_to_s3;


#[derive(Parser)]
struct Args {
    #[arg(short, long)]
    bucket_name_s3: String,
    #[arg(short, long)]
    key_s3: String,
}

// Training script entry point
// Steps
// 1. Download external CSV file to disk
// 2. Load file from disk into memory
// 3. Prepare the data
// 4. Train an XGBoost model with this data
// 5. Push this model to an AWS S3 bucket (model registry)
fn main() -> anyhow::Result<()> {
    println!("Starting training script...");

    // Parse command line arguments into a struct
    let args = Args::parse();

    // 1. Download external CSV file to disk
    let csv_file_path = download_csv_file()?;

    // 2. Load file from disk into memory
    let df = load_csv_file(&csv_file_path)?;

    // 3. Randomly split the data into training and testing sets
    let (train_df, test_df) = train_test_split(&df, 0.2)?;

    // 4. Split into features and target
    let (x_train, y_train) = split_features_and_target(&train_df)?;
    let (x_test, y_test) = split_features_and_target(&test_df)?;
    
    // 5. Train an XGBoost model with this data
    let path_to_model = train_xgboost_model(
        &x_train,
        &y_train,
        &x_test,
        &y_test
    )?;

    // 6. Push this model to an AWS S3 bucket (model registry)
    let bucket_name = args.bucket_name_s3;
    let key = args.key_s3;
    let runtime = tokio::runtime::Runtime::new()?;
    runtime.block_on(push_model_to_s3(
        &path_to_model,
        &bucket_name,
        &key
    ))?;
    println!("Model pushed to S3 bucket");
    Ok(())
}