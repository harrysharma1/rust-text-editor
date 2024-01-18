use std::env;
use std::io::{self, stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

pub struct Editor {
    should_exit: bool,

}

impl Editor {
    pub fn run( &mut self) {
        let _stdout = stdout().into_raw_mode().unwrap();

        loop {
            if let Err(error) = self.process_keypress() {
                error_handle(error);
            }

            if let Err(error) = self.refresh_screen() { 
                error_handle(error);
            }

            if self.should_exit{
                break;
            }
        }
    }
    pub fn default() -> Self {
        Self {should_exit: false,}

    }

    fn refresh_screen(&self) -> Result<(), std::io::Error> {
        let mut path = env::current_dir()?;
        path.push("assets/byebye.jpeg");
        
        let image  = image::open(
            &path
        ).expect("Image not found");

        let ascii_art = artem::convert(image, &artem::config::ConfigBuilder::new().build());
        let byebye = indoc::indoc! {"ByeBye"};
        print!("{}{}", termion::clear::All,termion::cursor::Goto(1,1));

        if self.should_exit{
            println!("{}",ascii_art);
            println!("{}",byebye);
        }
        io::stdout().flush()
    }

    fn process_keypress(&mut self) -> Result<(), std::io::Error> {
        let pressed_key = read_key()?;
        match pressed_key {
            Key::Ctrl('w') => self.should_exit = true,
            _ => (),
        }
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