# Let's Rust!

## Session 2

#### Table of contents


### Goals

- [x] Load file from disk into memory with Polars
    - [x] Install Polars with right features and version
    - [x] Polars LazyDatafram to load csv file
    - [x] Vec<i32> to randomly split into training data and testing data.
- [x] Randomly split into training and testing
- [x] Split features and targets
- [x] Train an XGBoost model with this data
- [x] Push this model to an AWS S3 bucket (model registry)
- [ ] Add instructions to set up the `aws cli` on your computer.

### Challenges

- Update the training parameters to train for more than 10 iterations.
    Hint: Check the xgboost crate documentation