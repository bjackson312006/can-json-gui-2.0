//! Parses a CAN JSON file into a Rust struct.

use std::fs;
use std::path::Path;
use crate::schema::{OdysseyMsg};
use crate::error::{ReadError, WriteError};

/// Parses a single CAN spec JSON file into a list of `OdysseyMsg` messages.
/// ### Params
/// * path - The path to the JSON file you want to parse.
pub(crate) fn read(path: impl AsRef<Path>) -> Result<Vec<OdysseyMsg>, ReadError> {
    let path = path.as_ref();
    let contents =
        fs::read_to_string(path).map_err(|e| ReadError::Io(path.display().to_string(), e))?;

    let de = &mut serde_json::Deserializer::from_str(&contents);
    serde_path_to_error::deserialize::<_, Vec<OdysseyMsg>>(de).map_err(|e| {
        ReadError::Deserialize(path.display().to_string(), e.path().to_string(), e.into_inner())
    })
}

/// Serializes a list of `OdysseyMsg` messages and writes them to a single spec JSON file.
/// ### Params
/// * path - The path to the JSON file you want to write. Any existing file is overwritten.
/// * msgs - The messages to serialize.
pub(crate) fn write(path: impl AsRef<Path>, msgs: &[OdysseyMsg]) -> Result<(), WriteError> {
    let path = path.as_ref();
    let contents = serde_json::to_string_pretty(msgs)
        .map_err(|e| WriteError::Serialize(path.display().to_string(), e))?;

    fs::write(path, contents).map_err(|e| WriteError::Io(path.display().to_string(), e))
}
