use std::env;
use termion::event::Key;
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
        
        Terminal::clear_screen();
        Terminal::cursor_pos(0, 0);

        if self.should_exit{
            println!("{}",buffer);
            println!("{}",byebye);
        }else{
           self.print_tilde();
           Terminal::cursor_pos(1, 1);
        }   
        Terminal::flush()
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
        let pressed_key = Terminal::read_key()?;
        match pressed_key {
            Key::Ctrl('w') => self.should_exit = true,
            _ => (),
        }
        Ok(())
    }
}


// Error handling using Rust IO error library
// If error does occur then the Terminal will Clear then the error will print
fn error_handle(e: std::io::Error) {
    Terminal::clear_screen();
    panic!("{}",e);
}