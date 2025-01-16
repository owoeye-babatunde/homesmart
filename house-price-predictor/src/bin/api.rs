use actix_web::{App, HttpServer, HttpResponse, Responder, get, post, web};
use log::{info, error};
use serde::{Serialize, Deserialize};
use clap::Parser;
use std::sync::Arc;
use xgboost::DMatrix;

use house_price_predictor::aws::download_model_from_s3;
use house_price_predictor::model::{load_xgboost_model, Model};

#[derive(Parser)]
struct Args {
    #[arg(short, long)]
    bucket_name_s3: String,
    #[arg(short, long)]
    key_s3: String,
    #[arg(short, long, default_value_t = 8080)]
    port: u16,
}

/// App state that will be shared across all workers of my actix server
#[derive(Clone)]
struct AppState {
    model: Arc<Model>,
}

/// Health check endpoint
/// Returns a 200 OK response if the API is healthy
/// with a message "I am healthy!"
#[get("/health")]
async fn health() -> impl Responder {
    info!("Health check endpoint called");
    // "Ok"
    HttpResponse::Ok().body("I am healthy!")
}

#[derive(Deserialize, Debug)]
struct PredictRequest {
    crim: f64,
    zn: f64,
    indus: f64,
    chas: f64,
    nox: f64,
    rm: f64,
    age: f64,
    dis: f64,
    rad: f64,
    tax: f64,
    ptratio: f64,
    b: f64,
    lstat: f64,
}

#[derive(Serialize)]
struct PredictResponse {
    prediction: f32,
}

/// Transform a JSON payload into a DMatrix
/// Returns an error if the transformation fails
fn transform_features_payload_to_dmatrix(payload: &web::Json<PredictRequest>) -> anyhow::Result<DMatrix> {
    
    // transform the payload into a slice of floating like &[f32]
    let features: Vec<f32> = [
        payload.crim,
        payload.zn,
        payload.indus,
        payload.chas,
        payload.nox,
        payload.rm,
        payload.age,
        payload.dis,
        payload.rad,
        payload.tax,
        payload.ptratio,
        payload.b,
        payload.lstat
    ].iter().map(|f| *f as f32).collect();

    let dmatrix_features = DMatrix::from_dense(
        &features, 1)?;

    Ok(dmatrix_features)
}

/// Predict endpoint
/// Accepts a JSON payload with features and returns a prediction
#[post("/predict")]
async fn predict(
    payload: web::Json<PredictRequest>,
    data: web::Data<AppState>
) -> impl Responder {
    info!("Predict endpoint called");

    // Just checking that the model is available as part of the app state
    // let model_metadata = data.model.get_attribute_names().unwrap();
    // info!("Model metadata: {:?}", model_metadata);
    info!("Features sent by the client:{:?}", payload);

    // Transform the payload into a DMatrix
    let dmatrix_features = transform_features_payload_to_dmatrix(&payload).unwrap();
    
    // Use the model and the `dmatrix_features` to generate a prediction
    let model = &data.model;
    let prediction = model.predict(&dmatrix_features).unwrap()[0];

    // build the response struct with the prediction
    let prediction_response = PredictResponse {
        prediction: prediction,
    };

    // Return the response as a JSON payload
    web::Json(prediction_response)
}

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    // Initialize the logger
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    
    // Parse command line arguments into a struct
    let args = Args::parse();

    // Download the model from S3 into a local file
    let model_path = download_model_from_s3(
        &args.bucket_name_s3,
        &args.key_s3
    ).await?;

    info!("Starting API...");

    HttpServer::new(move || {
        // Load the model into memory
        let model = load_xgboost_model(&model_path).unwrap();

        // Create the state data structure that will be shared across all workers
        let app_state = AppState {
            model: Arc::new(model),
        };

        App::new()
            .app_data(web::Data::new(app_state))
            .service(health)
            .service(predict)
    })
    .bind(("0.0.0.0", args.port))?
    .run()
    .await?;

    Ok(())
}
