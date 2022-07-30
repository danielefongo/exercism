use std::collections::HashMap;
use std::thread;
use std::thread::JoinHandle;

type Return = HashMap<char, usize>;

pub fn frequency(input: &[&str], workers: usize) -> Return {
    let mut map: Return = HashMap::new();
    input
        .chunks(input.len() / workers + 1)
        .map(join_inner_str)
        .map(|data| {
            thread::spawn(move || {
                let mut map: Return = HashMap::new();
                data.chars()
                    .filter(|it| it.is_alphabetic())
                    .map(|it| it.to_ascii_lowercase())
                    .for_each(|chr| *map.entry(chr).or_default() += 1);
                map
            })
        })
        .collect::<Vec<JoinHandle<Return>>>()
        .into_iter()
        .for_each(|thread| {
            thread
                .join()
                .unwrap()
                .into_iter()
                .for_each(|(chr, cnt)| *map.entry(chr).or_default() += cnt);
        });
    map
}

fn join_inner_str(chunk: &[&str]) -> String {
    chunk.iter().map(|it| (*it).to_string()).collect::<String>()
}
