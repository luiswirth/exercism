use std::collections::HashMap;
use std::sync::Arc;
use std::thread;

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let data = input
        .iter()
        .map(|s| s.to_lowercase())
        .collect::<Vec<String>>();
    let data = Arc::new(data);
    let mut threads = Vec::new();

    for i in 0..worker_count {
        let data = data.clone();
        threads.push(thread::spawn(move || {
            let mut map = HashMap::new();
            let mut k = i;
            while k < data.len() {
                for c in data.get(k).unwrap().chars().filter(|c| c.is_alphabetic()) {
                    *map.entry(c).or_insert(0) += 1;
                }
                k += worker_count;
            }
            map
        }));
    }

    let mut map = HashMap::new();
    for thread in threads {
        for (k, v) in thread.join().unwrap() {
            *map.entry(k).or_insert(0) += v;
        }
    }
    map
}
