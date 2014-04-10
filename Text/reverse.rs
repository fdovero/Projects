use std::io;

fn main() {
    for line in io::stdin().lines() {
        match line {
            Ok(line) => {  
                            // skip(1) skip the CRLF 
                            let s: ~str = line.chars_rev().skip(1).collect();
                            println!("{}\n", s);
                        },
            Err(err) => fail!("IO error: {}", err),
        }
    }
}
