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

        let mut buffer = String::new();
        rascii_art::render_to(
            &path.display().to_string(),
            &mut buffer,
            &rascii_art::RenderOptions::new()
                .width(25)
                .colored(true)
                .charset(&[".", ",", "-", "*", "£", "$", "#"]),
        ).unwrap(); 

        let byebye = indoc::indoc! {"Bye Bye !!!"};
        print!("{}{}", termion::clear::All,termion::cursor::Goto(1,1));

        if self.should_exit{
            println!("{}",buffer);
            println!("{}",byebye);
        }else{
           self.print_tilde();
           print!("{}", termion::cursor::Goto(1,1)); 
        }   
        io::stdout().flush()
    }

    fn print_tilde(&self){
        for _ in 0..24{
            println!("~\r");
        }
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
    println!("{}",termion::clear::All);
    panic!("{}",e);
}