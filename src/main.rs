#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod frontend;
mod backend;

macro_rules! log_err {
    ($expr:expr) => {
        if let Err(e) = $expr {
            eprintln!("Error in {}: {:?}", stringify!($expr), e);
        }
    };
}

// Main entry point for the application.
fn main() {
    frontend::start();
}