use spmc::{Receiver, Sender};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::thread;

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
    let num_thrs = 4;

    // Load dictionary
    let mut d = HashMap::new();
    for line in BufReader::new(File::open(dict_file).expect("Coult not read file")).lines() {
        d.insert(line.expect("There was a problem reading a line"), 0);
    }

    // Setup channel
    let (tx, rx): (Sender<Option<String>>, Receiver<Option<String>>) = spmc::channel();

    let mut handles = Vec::new();

    // Setup consumers for the channel
    for _ in 0..num_thrs {
        let rx = rx.clone();
        let dict = d.clone();
        handles.push(thread::spawn(move || work(rx.clone(), dict)));
    }

    // Send text lines to channel
    let input = File::open(&input_file).expect("Could not open input file");
    for l in BufReader::new(input).lines() {
        let _ = tx.send(Option::from(l.expect("Could not read line")));
    }

    // Send exit singals on the channel
    for _ in 0..num_thrs {
        let _ = tx.send(Option::None);
    }

    let mut re: HashMap<String, i32> = HashMap::new();
    // Wait for consumers to finish and merge their results
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

    // Print the top 10 results
    let mut res: Vec<(&String, &i32)> = re.iter().collect();
    res.sort_by(|(_, xv), (_, yv)| yv.cmp(xv));
    res.truncate(10);
    for (k, v) in res {
        println!("{} {}", k, v);
    }
}
