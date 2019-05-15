use spmc::{Receiver, Sender};
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::thread::JoinHandle;

fn work(
    rx: Receiver<Option<String>>,
    mut dict: HashMap<String, i32>,
    n: usize,
) -> Vec<(String, i32)> {
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
                None => return top_n(dict, n),
            }
        }
    }
}

fn top_n(dict: HashMap<String, i32>, n: usize) -> Vec<(String, i32)> {
    let mut res: Vec<(&String, &i32)> = dict.iter().filter(|(_k, v)| v.is_positive()).collect();
    res.sort_by(|(_, xv), (_, yv)| yv.cmp(xv));
    res.truncate(n);

    let mut out = Vec::new();
    for (k, v) in res {
        out.push((k.clone(), v.clone()));
    }
    return out;
}

fn setup_consumers(
    d: HashMap<String, i32>,
    rx: Receiver<Option<String>>,
    handles: &mut Vec<JoinHandle<Vec<(String, i32)>>>,
    num_thrs: &usize,
    n: usize,
) {
    for _ in 0..*num_thrs {
        let rx = rx.clone();
        let dict = d.clone();
        handles.push(std::thread::spawn(move || work(rx.clone(), dict, n)));
    }
}

fn merge_results(handles: Vec<JoinHandle<Vec<(String, i32)>>>) -> Vec<(String, i32)> {
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
    let mut res: Vec<(&String, &i32)> = re.iter().collect();
    res.sort_by(|(_, xv), (_, yv)| yv.cmp(xv));
    res.truncate(10);
    // There has to be a better way
    res.iter()
        .map(|(x, y)| (x.clone().clone(), y.clone().clone()))
        .collect()
}

fn load_dict(d: &mut HashMap<String, i32>, dict_file: &String) {
    for line in BufReader::new(File::open(dict_file).expect("Coult not read file")).lines() {
        d.insert(line.expect("There was a problem reading a line"), 0);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let dict_file = &args[1];
    let input_file = &args[2];
    let num_thrs = &args[3]
        .parse::<usize>()
        .expect("Failed to parse number of threads");
    let n = 10;

    // Load dictionary
    let mut d = HashMap::new();
    load_dict(&mut d, dict_file);

    // Setup channel
    let (tx, rx): (Sender<Option<String>>, Receiver<Option<String>>) = spmc::channel();

    let mut handles = Vec::new();
    setup_consumers(d, rx, &mut handles, &num_thrs, n);

    // Send text lines to channel
    let input = File::open(&input_file).expect("Could not open input file");
    for l in BufReader::new(input).lines() {
        let _ = tx.send(Option::from(l.expect("Could not read line")));
    }

    // Send exit singals on the channel
    for _ in 0..*num_thrs {
        let _ = tx.send(Option::None);
    }

    let res = merge_results(handles);
    for (k, v) in res {
        println!("{} {}", k, v);
    }
}
