use regex::Regex;
use std::env;
use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();

    let args: Vec<String> = env::args().collect();
    assert!(args.len() == 2);

    let re = Regex::new(&args[1]).unwrap();

    for line in stdin.lock().lines() {
        let line = &line.unwrap();
        let mut replaced = line.clone();
        let mut offset: usize = 0;
        for c in re.captures_iter(line) {
            for m in c.iter() {
                let mu = m.unwrap();
                let colored = format!("\x1b[93m{}\x1b[0m", mu.as_str());
                let start = mu.start() + offset;
                let end = mu.end() + offset;
                replaced.replace_range(start..end, &colored);

                offset += colored.len() - (mu.end() - mu.start());
            }
        }
        println!("{}", replaced);
    }
}
