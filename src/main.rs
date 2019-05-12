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

fn lines(file: &String) -> std::io::Lines<std::io::BufReader<std::fs::File>> {
    match File::open(file) {
        Ok(file) => BufReader::new(file).lines(),
        Err(err) => panic!("Could not open file, {:?}", err),
    }
}

fn work(rx: Receiver<Option<String>>, mut dict: HashMap<String, i32>) -> Vec<(String, i32)> {
    loop {
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
                    let mut res: Vec<(&String, &i32)> =
                        dict.iter().filter(|(_k, v)| v.is_positive()).collect();
                    res.sort_by(|(_, xv), (_, yv)| yv.cmp(xv));
                    res.truncate(10);

                    let mut out = Vec::new();
                    for (k, v) in res {
                        out.push((k.clone(), v.clone()));
                    }
                    return out;
                }
            }
        }
    }
}

fn main() {
    let dict_file = String::from("/home/kiril/tmp/wordcounting/words_alpha.txt");
    let input_file = String::from("/home/kiril/tmp/wordcounting/small.txt");

    let mut handles = Vec::new();
    let num_thrs = 4;

    // Load dictionary
    let d = load_dict(&dict_file);

    // Setup channel
    let (tx, rx): (Sender<Option<String>>, Receiver<Option<String>>) = spmc::channel();

    // Setup consumers
    for _ in 0..num_thrs {
        let rx = rx.clone();
        let dict = d.clone();
        handles.push(thread::spawn(move || work(rx.clone(), dict)));
    }

    // Send lines to threads
    for l in lines(&input_file) {
        let _ = tx.send(Option::from(l.expect("Line was not there")));
    }

    // Send exit singal to threads
    for _ in 0..num_thrs {
        let _ = tx.send(Option::None);
    }

    // Wait for threads to finish and print their local result
    // let mut res: Vec<(String, i32)> = Vec::new();
    let mut re: HashMap<String, i32> = HashMap::new();
    for handle in handles {
        let thr_res = handle.join().unwrap();
        for (k, v) in thr_res {
            match re.get(&k) {
                Some(ev) => {
                    re.insert(k, ev + v);
                }
                None => {
                    re.insert(k, v);
                }
            }
        }
    }
    let mut res: Vec<(&String, &i32)> = re.iter().collect();
    res.sort_by(|(_, xv), (_, yv)| yv.cmp(xv));
    res.truncate(10);

    for (k, v) in res {
        println!("{} {}", k, v);
    }
}
