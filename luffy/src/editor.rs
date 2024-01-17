use std::io::{self, stdout};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

pub struct Editor {}

impl Editor {
    pub fn run(&self) {
        let _stdout = stdout().into_raw_mode().unwrap();

        loop {
            if let Err(error) = self.process_keypress() {
                error_handle(error);
            }
        }
    }
    pub fn default() -> Self {
        Self {}
    }
    fn process_keypress(&self) -> Result<(), std::io::Error> {
        let pressed_key = read_key()?;
        if let Key::Ctrl('q') = pressed_key { panic!("Program end") }
        Ok(())
    }
}

fn read_key() -> Result<Key, std::io::Error> {
    loop {
        if let Some(key) = io::stdin().lock().keys().next() {
            return key;
        }
    }
}

fn error_handle(e: std::io::Error) {
    panic!("{}",e);
}