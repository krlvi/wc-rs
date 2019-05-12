use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::thread;

fn load_dict(dict_file: String) -> HashMap<String, i32> {
    let mut dict = HashMap::new();

    match File::open(dict_file) {
        Ok(file) => {
            for line in BufReader::new(file).lines() {
                match line {
                    Ok(line) => {
                        dict.insert(line, 0);
                    }
                    Err(error) => panic!("There was a problem reading a line: {:?}", error),
                }
            }
        }
        Err(error) => panic!("There was a problem opening the file: {:?}", error),
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
                                }
                                None => {}
                            }
                        }
                    }
                    Err(error) => panic!("there was a problem reading a line: {:?}", error),
                }
            }
        }
        Err(error) => panic!("There was a problem opening the input file {:?},", error),
    }
}
fn lines(file: String) -> std::io::Lines<std::io::BufReader<std::fs::File>> {
    match File::open(file) {
        Ok(file) => BufReader::new(file).lines(),
        Err(err) => panic!("Could not open file, {:?}", err),
    }
}

fn main() {
    //    let dict = load_dict(String::from("/home/kiril/tmp/wordcounting/words_alpha.txt"));
    // count_words(&mut dict, small);
    //    count_words(&mut dict, String::from("/home/kiril/tmp/wordcounting/14m_hn_comments_sorted.txt"));

    /*
    let mut res: Vec<(&String, &i32)> = dict.iter()
        .filter(|(_k, v)| v.is_positive())
      //  .map(|(k, v)| (k, v))
        .collect();
    res.sort_by(|(_, xv), (_, yv)| yv.cmp(xv));
    res.truncate(10);
    */

    let mut handles = Vec::new();
    let (tx, rx): (
        spmc::Sender<std::result::Result<std::string::String, std::io::Error>>,
        spmc::Receiver<std::result::Result<std::string::String, std::io::Error>>,
    ) = spmc::channel();

    for n in 0..4 {
        let rx = rx.clone();
        handles.push(thread::spawn(move || {
            let mut lines = 0;
            let mut dict = load_dict(String::from("/home/kiril/tmp/wordcounting/words_alpha.txt"));
            loop {
                for msg in rx.recv().iter() {
                    match msg {
                        Ok(line) => {
                            for word in line.split_whitespace() {
                                match dict.get(word) {
                                    Some(val) => {
                                        dict.insert(String::from(word), *val + 1);
                                    }
                                    None => {}
                                }
                            }
                        }
                        Err(err) => {
                            return dict;
                        }
                    }
                }
            }
        }));
    }

    for l in lines(String::from(
        "/home/kiril/tmp/wordcounting/14m_hn_comments_sorted.txt",
    )) {
        //for l in lines(String::from("/home/kiril/tmp/wordcounting/small.txt")) {
        tx.send(l);
    }
    tx.send(Result::Err(std::io::Error::from_raw_os_error(10022)));
    tx.send(Result::Err(std::io::Error::from_raw_os_error(10022)));
    tx.send(Result::Err(std::io::Error::from_raw_os_error(10022)));
    tx.send(Result::Err(std::io::Error::from_raw_os_error(10022)));

    for handle in handles {
        println!("{:?}", handle.join().unwrap().len());
    }
    /*
    for (k, v) in res {
        println!("{} {}", k, v);
    }
    */
}
