//! Jason

pub mod error;
pub mod schema;
mod parse;

use std::path::PathBuf;

use crate::{error::{ OpenError, ReadError, SaveAsError, SaveError, WriteError, NewError }, schema::OdysseyMsg};

/// Struct representing a CAN JSON file.
#[derive(Debug)]
pub struct CanJson {
    /// The path of the JSON file.
    path: PathBuf,

    /// The messages held inside the JSON file.
    messages: Vec<schema::OdysseyMsg>,
}

impl CanJson {
    /// Reads in a `.json` file from a raw path and parses it into a `CanJson` struct instance.
    /// ### Parameters
    /// * `path` - The filepath of the `.json` to read in.
    pub fn read(path: PathBuf) -> Result<Self, ReadError> {
        let messages: Vec<schema::OdysseyMsg> = parse::read(&path)?;
        Ok(Self { path, messages, })
    }

    /// The messages contained in this file.
    pub fn messages(&self) -> &[OdysseyMsg] {
        &self.messages
    }

    /// Gets a message from an index.
    pub fn message(&self, index: usize) -> &OdysseyMsg {
        &self.messages[index]
    }

    /// Removes the message at `index` from this file.
    pub fn remove_message(&mut self, index: usize) {
        if index < self.messages.len() {
            self.messages.remove(index);
        }
    }

    /// Adds a new message to the end of the JSON.
    pub fn add_message(&mut self) {
        self.messages.push(OdysseyMsg::default())
    }

    /// Writes a `CanJson` instance into a `.json` file.
    pub fn write(&self) -> Result<(), WriteError> {
        parse::write(&self.path, self.messages.as_slice())
    }

    /// Opens a `.json` file via the OS's dialog box.
    pub fn open() -> Result<CanJson, OpenError> {
        let path = match rfd::FileDialog::new().add_filter("json", &["json"]).pick_file() {
            Some(path) => path,
            None => { return Err(OpenError::Cancelled); }
        };

        match CanJson::read(path) {
            Ok(file) => Ok(file),
            Err(error) => Err(OpenError::ReadError(error)),
        }
    }

    /// Saves a file to its path.
    pub fn save(&self) -> Result<(), SaveError> {
        match self.write() {
            Ok(_) => Ok(()),
            Err(err) => Err(SaveError::WriteError(err))
        }
    }

    /// Save-as for a file. Basically just `save()` but the user picks a new path via a dialogue box.
    pub fn save_as(&mut self) -> Result<(), SaveAsError> {
        let path = match rfd::FileDialog::new().add_filter("json", &["json"]).save_file() {
            Some(path) => path,
            None => { return Err(SaveAsError::Cancelled); }
        };

        self.path = path;
        match self.write() {
            Ok(_) => Ok(()),
            Err(err) => Err(SaveAsError::WriteError(err))
        }
    }

    /// Allows the user to create a new empty CAN JSON via a OS dialogue box.
    pub fn new() -> Result<Self, NewError> {
        let path = match rfd::FileDialog::new().add_filter("json", &["json"]).save_file() {
            Some(path) => path,
            None => { return Err(NewError::Cancelled); }
        };

        let json = CanJson {
            path: path,
            messages: Vec::new(),
        };

        match json.write() {
            Ok(_) => (),
            Err(err) => { return Err(NewError::WriteError(err)); }
        }

        Ok( json )
    }
}