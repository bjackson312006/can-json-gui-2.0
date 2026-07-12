//! Module for any error types in the `json` crate.

use thiserror::Error;

/// Errors that can occur when trying to open a CAN JSON.
#[derive(Error, Debug)]
pub enum OpenError {
    #[error("The user canceled/exited the open dialog without choosing a file.")]
    Cancelled,

    #[error("A read error occured while trying to open a file: {0}")]
    ReadError(ReadError),
}

/// Errors that can occur when trying to save a CAN JSON.
#[derive(Error, Debug)]
pub enum SaveError {
    #[error("A write error occured while trying to save a file: {0}")]
    WriteError(WriteError),
}

/// Errors that can occur when trying to save-as a CAN JSON.
#[derive(Error, Debug)]
pub enum SaveAsError {
    #[error("The user canceled/exited the save-as dialog without location/path to save the file at.")]
    Cancelled,

    #[error("A write error occured while trying to save-as a file: {0}")]
    WriteError(WriteError),
}

/// Errors that can occur when trying to create a new CAN JSON.
#[derive(Error, Debug)]
pub enum NewError {
    #[error("The user canceled/exited the new-file dialog without location/path to create the file at.")]
    Cancelled,

    #[error("A write error occured while trying to create a new file: {0}")]
    WriteError(WriteError),
}

/// Errors that can occur when reading in a CAN JSON file.
#[derive(Error, Debug)]
pub enum ReadError {
    #[error("Could not read spec path {0}")]
    Io(String, #[source] std::io::Error),

    #[error("Could not deserialize spec file {0} (at {1})")]
    Deserialize(String, String, #[source] serde_json::Error),
}

/// Errors that can occur while writing to CAN JSON files.
#[derive(Error, Debug)]
pub enum WriteError {
    #[error("Could not read spec path {0}")]
    Io(String, #[source] std::io::Error),

    #[error("Could not serialize messages to {0}")]
    Serialize(String, #[source] serde_json::Error),
}