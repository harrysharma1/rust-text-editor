use std::io;
use std::io::Read;
use std::io::stdout;
use termion::raw::IntoRawMode;


fn main() {
    let _output = stdout().into_raw_mode().unwrap();

    for b in std::io::stdin().bytes() {
        let b: u8 = b.unwrap();
        let c: char = b as char;
        println!("{}",c);
        if c.is_control(){
            print!("{:?}\r",b);
        }else{
            println!("{:?} ({})\r", b, c);
        }

    }
}
