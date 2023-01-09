#![warn(clippy::all, rust_2018_idioms)]

mod app;
mod image;
mod gallery;
mod editor;

pub use app::TemplateApp;
pub use crate::image::WebImage;
pub use gallery::Gallery;
pub use editor::Editor;