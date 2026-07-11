//! Parses a CAN JSON file into a Rust struct.

use std::fs;
use std::path::Path;
use thiserror::Error;
use crate::schema::{OdysseyMsg, CANMsg};

/**
 *  Errors that can occur while parsing CAN spec JSON files
 */
#[derive(Error, Debug)]
pub enum ParseError {
    #[error("Could not read spec path {0}")]
    Io(String, #[source] std::io::Error),

    #[error("Could not deserialize spec file {0} (at {1})")]
    Deserialize(String, String, #[source] serde_json::Error),
}

/**
 *  Parse a single CAN spec JSON file into a list of `OdysseyMsg`.
 *  Uses `serde_path_to_error` so failures report the offending path
 *  within the JSON document.
 */
pub fn parse_spec_file(path: impl AsRef<Path>) -> Result<Vec<OdysseyMsg>, ParseError> {
    let path = path.as_ref();
    let contents =
        fs::read_to_string(path).map_err(|e| ParseError::Io(path.display().to_string(), e))?;

    let de = &mut serde_json::Deserializer::from_str(&contents);
    serde_path_to_error::deserialize::<_, Vec<OdysseyMsg>>(de).map_err(|e| {
        ParseError::Deserialize(path.display().to_string(), e.path().to_string(), e.into_inner())
    })
}

/**
 *  Parse every `*.json` spec file in `dir` into a flat list of `OdysseyMsg`.
 */
pub fn parse_spec_dir(dir: impl AsRef<Path>) -> Result<Vec<OdysseyMsg>, ParseError> {
    let dir = dir.as_ref();
    let mut msgs = Vec::new();

    let entries = fs::read_dir(dir).map_err(|e| ParseError::Io(dir.display().to_string(), e))?;
    for entry in entries {
        let path = entry
            .map_err(|e| ParseError::Io(dir.display().to_string(), e))?
            .path();
        if path.is_file() && path.extension().is_some_and(|ext| ext == "json") {
            msgs.extend(parse_spec_file(&path)?);
        }
    }

    Ok(msgs)
}

/**
 *  Convenience wrapper that keeps only the `CANMsg` variants,
 *  discarding `MetaMsg` entries.
 */
pub fn parse_can_msgs(dir: impl AsRef<Path>) -> Result<Vec<CANMsg>, ParseError> {
    Ok(parse_spec_dir(dir)?
        .into_iter()
        .filter_map(|msg| match msg {
            OdysseyMsg::Can(can) => Some(can),
            OdysseyMsg::Meta(_) => None,
        })
        .collect())
}
