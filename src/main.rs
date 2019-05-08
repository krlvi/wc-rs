use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

fn load_dict(dict_file: String) -> HashMap<String, i32> {
    let mut dict : HashMap<String, i32> = HashMap::new();

    match File::open(dict_file) {
        Ok(file) => {
            for line in BufReader::new(file).lines() {
                match line {
                    Ok(line) => {
                        dict.insert(line, 1);
                    },
                    Err(error) => {
                        panic!("There was a problem reading a line: {:?}", error)
                    }
                }
            }
        },
        Err(error) => {
            panic!("There was a problem opening the file: {:?}", error)
        }
    }
    dict
}
fn main() {
    let dict = load_dict(String::from("/home/kiril/tmp/wordcounting/words_alpha.txt"));
    println!("{}", dict.len());
}
