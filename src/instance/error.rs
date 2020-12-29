use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Failed to open connection to display named: {0}")]
    OpenDisplay(String),
}
