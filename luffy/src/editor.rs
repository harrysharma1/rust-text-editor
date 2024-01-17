use std::io::{self,stdout};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
    
pub struct Editor{}


impl Editor{
    pub fn run(&self){
        let _output = stdout().into_raw_mode().unwrap();
        for key in io::stdin().keys(){
            match key{
                Ok(key)=> match key {
                    Key::Char(c) =>{
                        if c.is_control(){
                            print!("{:?} \r",c as u8);
                        }else{
                            println!("{:?} ({})  \r", c as u8, c);
                        }
                    }
                    Key::Ctrl('w') => break,
                    _=> println!("{:?} \r", key)
                }
    
                
                Err(err) => error_handle(err)
    
            }
        }
    }
    pub fn default() -> Self{
        Self{}
    }
}
fn error_handle(e: std::io::Error){
    panic!("{}", e);
}