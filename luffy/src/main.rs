use std::io;
use std::io::Read;
use std::io::stdout;
use termion::raw::IntoRawMode;


fn main() {
    let _output = stdout().into_raw_mode().unwrap();

    for b in std::io::stdin().bytes() {
        let c = b.unwrap() as  char;
        println!("{}",c);
        if c == 'q'{
            break;
        }
    }
}
