#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod frontend;
mod backend;

// Main entry point for the application.
fn main() {
    frontend::start();
}