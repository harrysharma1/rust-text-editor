use std::io;
use std::io::Read;
use std::io::stdout;
use termion::raw::IntoRawMode;


fn ctrl_to_char(c:char)->u8{
    let byte = c as u8;
    byte & 0b00011111
}
fn main() {
    let _output = stdout().into_raw_mode().unwrap();

    for b in std::io::stdin().bytes() {
        let b: u8 = b.unwrap();
        let c: char = b as char;
       
        if c.is_control(){
            print!("{:?} \r",b);
        }else{
            println!("{:?} ({})\r", b, c);
        }

        if b == ctrl_to_char('w'){
            break;
        }

    }
}
