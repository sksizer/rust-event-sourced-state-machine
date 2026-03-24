use thiserror::Error;

#[derive(Error, Debug)]
pub enum StepError {
    #[error("Invalid Configuration: {0}")]
    InvalidConfig(String),
}