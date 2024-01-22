#![warn(clippy::all, clippy::pedantic)]
mod editor;
mod terminal;
mod row;
mod doc;
use editor::Editor; 
pub use terminal::Terminal;
pub use editor::Position;
pub use row::Row;
pub use doc::Document;
fn main() {
    Editor::default().run();
    
}
