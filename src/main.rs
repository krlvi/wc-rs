use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

fn load_dict(dict_file: String) -> HashMap<String, i32> {
    let mut dict = HashMap::new();

    match File::open(dict_file) {
        Ok(file) => {
            for line in BufReader::new(file).lines() {
                match line {
                    Ok(line) => {
                        dict.insert(line, 0);
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

fn count_words(dict: &mut HashMap<String, i32>, input_file: String) {
    match File::open(input_file) {
        Ok(file) => {
            for line in BufReader::new(file).lines() {
                match line {
                    Ok(line) => {
                        for word in line.split_whitespace() {
                            match dict.get(word) {
                                Some(val) => {
                                    dict.insert(String::from(word), *val + 1);
                                },
                                None => {}
                            }
                        }
                    },
                    Err(error) => {
                        panic!("there was a problem reading a line: {:?}", error)
                    }
                }
            }

        },
        Err(error) => {
            panic!("There was a problem opening the input file {:?},", error)
        }
    }
}
fn main() {
    let mut dict = load_dict(String::from("/home/kiril/tmp/wordcounting/words_alpha.txt"));
  //  count_words(&mut dict, String::from("/home/kiril/tmp/wordcounting/small.txt"));
    count_words(&mut dict, String::from("/home/kiril/tmp/wordcounting/14m_hn_comments_sorted.txt"));

    let mut res: Vec<(&String, &i32)> = dict.iter()
        .filter(|(_k, v)| v.is_positive())
      //  .map(|(k, v)| (k, v))
        .collect();
    res.sort_by(|(_, xv), (_, yv)| yv.cmp(xv));
    res.truncate(10);

    for (k, v) in res {
        println!("{} {}", k, v);
    }
}
