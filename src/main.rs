use spmc::{Receiver, Sender};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::thread;

fn load_dict(dict_file: &String) -> HashMap<String, i32> {
    let mut dict = HashMap::new();
    for line in BufReader::new(File::open(dict_file).expect("Coult not read file")).lines() {
        dict.insert(line.expect("There was a problem reading a line"), 0);
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

    let dict_file = String::from("/home/kiril/tmp/wordcounting/words_alpha.txt");

    let mut handles = Vec::new();
    let num_thrs = 4;

    let (tx, rx): (Sender<Option<String>>, Receiver<Option<String>>) = spmc::channel();

    let d = load_dict(&dict_file);

    for t in 0..num_thrs {
        let rx = rx.clone();
        let mut dict = d.clone();
        handles.push(thread::spawn(move || loop {
            for msg in rx.recv().iter() {
                match msg {
                    Some(line) => {
                        for word in line.split_whitespace() {
                            match dict.get(word) {
                                Some(val) => {
                                    dict.insert(String::from(word), *val + 1);
                                }
                                None => {}
                            }
                        }
                    }
                    None => {
                        return dict;
                    }
                }
            }
        }));
    }

    // Send lines to threads
    for l in lines(String::from("/home/kiril/tmp/wordcounting/small.txt")) {
        let _ = tx.send(Option::from(l.expect("Line was not there")));
    }

    // Send exit singal to threads
    for _ in 0..num_thrs {
        let _ = tx.send(Option::None);
    }

    // Wait for threads to finish and print their local result
    for handle in handles {
        println!("{:?}", handle.join().unwrap().len());
    }
    /*
    for (k, v) in res {
        println!("{} {}", k, v);
    }
    */
}
