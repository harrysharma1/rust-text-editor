use std::io;
use std::io::Read;
fn main() {

    for b in std::io::stdin().bytes() {
        let c = b.unwrap() as  char;
        println!("{}",c);
        if c == 'w'{
            break;
        }
    }
}
