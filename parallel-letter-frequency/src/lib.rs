use std::collections::HashMap;
use std::sync::mpsc;
use std::thread;

fn batch_size(lines_number: usize, group_number: usize) -> usize {
    if group_number <= 1 {
        return lines_number;
    }
    lines_number / group_number + if lines_number % group_number != 0 {1} else {0}
}

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let words: Vec<String> = input
        .join(" ")
        .split_whitespace()
        .map(|s|s.to_string().to_ascii_lowercase())
        .collect();
    if words.len() == 0 {
        return HashMap::new();
    }
    let batch_size = batch_size(words.len(), worker_count);
    let chunks = words.chunks(batch_size);

    let mut handlers = vec![];
    let (send, recv) = mpsc::channel();

    for chunk in chunks {
        let line = chunk.concat();
        let sender = send.clone();
        let handler = thread::spawn(move || {
            let mut map: HashMap<char, usize> = HashMap::new();
            line.chars()
                .filter(|c|c.is_alphabetic())
                .for_each(|c|{
                *map.entry(c.to_ascii_lowercase()).or_insert(0) += 1;
            });
            sender.send(map).unwrap();
        });
        handlers.push(handler)
    }
    drop(send);

    let mut result: HashMap<char, usize> = HashMap::new();
    for map in recv {
        map.iter().for_each(|(&k, &v)|{
            *result.entry(k).or_insert(0) += v;
        });
    }
    result
}

