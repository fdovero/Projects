use std::io;


fn piglatin(source: ~str) -> ~str {
    let voyels = ~"aeiouy";
    let s: ~str =
        if voyels.contains_char(source.char_at(0)) {
            source.slice(0,source.len()-2).trim() + "way"
        } else {
            source.slice(1,source.len()-2).trim() + source.slice(0,1) + "ay" 
        };

    return s;
}

fn main() {
    for line in io::stdin().lines() {
        match line {
            Ok(line) => println!("{}\n", piglatin(line)),
            Err(err) => fail!("IO error: {}", err),
        }
    } 
} 
