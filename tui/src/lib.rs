//! Terminal front-end for Terminal Theatre: screens, drawing and effects on top
//! of `theatre-engine`. The `theatre` binary (`main.rs`) only parses arguments
//! and runs the loop; tests drive [`app::App`] the same way.

pub mod app;
pub mod render;
pub mod screens;
