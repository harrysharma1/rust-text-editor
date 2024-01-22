use termion::event::Key;
use crate::Terminal;
use crate::Document;

const VERSION: &str  = env!("CARGO_PKG_VERSION");



#[derive(Default)]
pub struct Position{
    pub x:usize,
    pub y:usize,
}

// Editor Struct that encompasses where the processing part of the editor
// - should_exit : Boolean value that breaks loop when true
// - terminal : Terminal struct that gives the dimensions 
pub struct Editor {
    should_exit: bool,
    terminal: Terminal,
    cursor_pos: Position,
    doc: Document,
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
            cursor_pos:Position::default(),
            doc: Document::open(),
        }

    }

    // Refreshing the screen after every keypress
    // Once you start typing terminal clears
    // It then checks if the user has exited. 
    // If they have it clears terminal with good bye message and ASCII art.
    // If they haven't then it will print ~ like with Vim
    // ASCII art is not formatted properly as of now
    fn refresh_screen(&self) -> Result<(), std::io::Error> {
        
        // let mut path = env::current_dir()?;
        // path.push("assets/byebye.jpeg");

        // let mut buffer = String::new();
        
        // rascii_art::render_to(
        //     &path.display().to_string(),
        //     &mut buffer,
        //     &rascii_art::RenderOptions::new()
        //         .width(25)
        //         .colored(true)
        //         .charset(&[".", ",", "-", "*", "£", "$", "#"]),
        // ).unwrap(); 

        let byebye = indoc::indoc! {"Bye Bye !!!"};
        
        Terminal::cursor_hide();
        Terminal::cursor_pos(&Position::default());

        if self.should_exit{
            Terminal::clear_screen();
            println!("{}",byebye);
        }else{
           self.print_tilde();
           Terminal::cursor_pos(&self.cursor_pos);
        }

        Terminal::cursor_show();   
        Terminal::flush()
    }

    // Simple loop to print tilde based on terminal height
    fn print_tilde(&self){
        let height = self.terminal.size().height;
        for row in 0.. height-1{
            Terminal::clear_current_line();
            if row == height/3{
                self.welcome_message();
            }else{
                print!("~\r");
            }
        }
    }

    // Processing keys with escape keys being matched to a function
    // For now only Ctrl+w exits the terminal
    fn process_keypress(&mut self) -> Result<(), std::io::Error> {
        let pressed_key = Terminal::read_key()?;
        match pressed_key {
            Key::Ctrl('w') => self.should_exit = true,
            Key::Up 
            |Key::Down 
            |Key::Left 
            |Key::Right 
            |Key::PageUp
            |Key::PageDown
            |Key::End
            |Key::Home => self.move_cursor(pressed_key),
            _ => (),
        }
        Ok(())
    }

    fn move_cursor(&mut self, key:Key){
        let Position{mut x, mut y} = self.cursor_pos;
        let size = self.terminal.size();
        let height = size.height.saturating_sub(1) as usize;
        let width = size.width.saturating_sub(1) as usize;
        match key{
            Key::Up => y = y.saturating_sub(1),
            Key::Down => {
                if y < height {
                    y = y.saturating_add(1);
                }
            }
            Key::Left => x = x.saturating_sub(1),
            Key::Right => {
                if x < width {
                    x = x.saturating_add(1);
                }
            }
            Key::PageUp => y=0,
            Key::PageDown => y = height,
            Key::Home => x=0,
            Key::End => x = width,
            _ => (),
        }

        self.cursor_pos = Position{x,y};
    }

    fn welcome_message(&self){
        let mut welcome = format!("Luffy's editor --v{}--",VERSION);
        let width = self.terminal.size().width as usize;
        let len  = welcome.len();
        let padding = width.saturating_sub(len)/2;
        let spacing  = " ".repeat(padding.saturating_sub(1));
        welcome = format!("~{}{}", spacing, welcome);            
        welcome.truncate(width);            
        println!("{}\r", welcome);  

    }
}


// Error handling using Rust IO error library
// If error does occur then the Terminal will Clear then the error will print
fn error_handle(e: std::io::Error) {
    Terminal::clear_screen();
    panic!("{}",e);
}