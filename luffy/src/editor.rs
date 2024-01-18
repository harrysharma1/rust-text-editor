use std::env;
use std::io::{self, stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use crate::Terminal;


// Editor Struct that encompasses where the processing part of the editor
// - should_exit : Boolean value that breaks loop when true
// - terminal : Terminal struct that gives the dimensions 
pub struct Editor {
    should_exit: bool,
    terminal: Terminal,
}


// Implement all processing logic of the Editor
impl Editor {
    // Creates the loop that keeps the editor running
    // After the keypress is error checked, it will: 
    // 1. Process the keypress
    // 2. Refresh terminal screen 
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

    // Default values for struct when it is created to be used
    pub fn default() -> Self {
        Self {
            should_exit: false,
            terminal: Terminal::default().expect("Failed to launch terminal"),

        }

    }

    // Refreshing the screen after every keypress
    // Once you start typing terminal clears
    // It then checks if the user has exited. 
    // If they have it clears terminal with good bye message and ASCII art.
    // If they haven't then it will print ~ like with Vim
    // ASCII art is not formatted properly as of now
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

    // Simple loop to print tilde based on terminal height
    fn print_tilde(&self){
        for _ in 0.. self.terminal.size().height{
            println!("~\r");
        }
    }

    // Processing keys with escape keys being matched to a function
    // For now only Ctrl+w exits the terminal
    fn process_keypress(&mut self) -> Result<(), std::io::Error> {
        let pressed_key = read_key()?;
        match pressed_key {
            Key::Ctrl('w') => self.should_exit = true,
            _ => (),
        }
        Ok(())
    }
}

// Loops and takes key inputs
fn read_key() -> Result<Key, std::io::Error> {
    loop {
        if let Some(key) = io::stdin().lock().keys().next() {
            return key;
        }
    }
}

// Error handling using Rust IO error library
// If error does occur then the Terminal will Clear then the error will print
fn error_handle(e: std::io::Error) {
    println!("{}",termion::clear::All);
    panic!("{}",e);
}