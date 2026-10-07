//! Terminal Theatre engine: everything about stories that is not drawing.
//!
//! A story is a self-contained folder:
//!
//! ```text
//! stories/<id>/
//!   story.toml          title, description, start scene, speakers, themes
//!   scenes/*.toml       scene tables, any number of files (e.g. one per chapter)
//!   characters/*.toml   pixel-art sprites, referenced by file name
//! ```
//!
//! [`library::discover`] lists story folders, [`pack::StoryPack::load`] loads and
//! validates one, and [`runner::Runner`] plays it. Front-ends only render.

pub mod color;
pub mod library;
pub mod pack;
pub mod progress;
pub mod rng;
pub mod runner;
pub mod save;
pub mod scene;
pub mod settings;
pub mod sprite;
pub mod state;
pub mod store;

pub use color::Rgb;
pub use pack::StoryPack;
pub use rng::Rng;
pub use store::Store;
