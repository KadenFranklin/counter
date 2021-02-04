use std::{env, io, fs};

fn main() -> io::Result<()>{
    for file in env::args().skip(1) {
        let variable = file.chars().clone();
        for char in variable.clone() {
            if char.clone() == '-' {
                for file in env::args().skip(2) {
                    let file_contents = fs::read_to_string(&file)?;
                    let words = file_contents.split_whitespace();
                    let mut int = 0;
                    let mut wrd = 0;
                    for word in words{
                        wrd = wrd + 1;
                        int = int + word.len();
                    }
                    for char in variable.clone(){
                        if char.clone() == 'w' {
                            println!("words:{}", wrd )
                        }
                    }
                    for char in variable.clone(){
                        if char.clone() == 'l' {
                            let line_count = file_contents.lines().count();
                            println!("lines:{}", line_count)
                        }
                    }
                    for char in variable.clone() {
                        if char.clone() == 'c' {
                            let character_count = int;
                            println!("characters:{}", character_count)
                        }
                    }
                }
            }
        }
    }
    Ok(())
}