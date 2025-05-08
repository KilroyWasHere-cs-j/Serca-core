// lib.rs for the `serca` library

pub mod ai;
pub mod plugin_manager;
pub mod web;

/// A simple greeting function for testing.
pub fn hello() -> &'static str {
    "Hello from serca!"
}
