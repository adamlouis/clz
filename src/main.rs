use regex::{Match, Regex};
use std::env;
use std::io::{self, BufRead};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 || args.len() > 3 {
        println!("usage: clr <regex> [black|red|green|yellow|blue|magenta|cyan|white]");
        return;
    };

    let mut color = Color::Green;
    if args.len() == 3 {
        match parse_color(args[2].as_str()) {
            Some(c) => color = c,
            None => {
                println!("invalid color - supported options are: black|red|green|yellow|blue|magenta|cyan|white");
                return;
            }
        }
    }

    let re = Regex::new(&args[1]).unwrap();
    if re.captures_len() > 1 {
        println!("error: regex must not contain capture groups");
        return;
    }

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let mut line = line.unwrap();
        color_line(&mut line, &re, &color);
        println!("{}", line);
    }
}

enum Color {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
}

fn parse_color(arg: &str) -> Option<Color> {
    match arg {
        "black" => Some(Color::Black),
        "red" => Some(Color::Red),
        "green" => Some(Color::Green),
        "yellow" => Some(Color::Yellow),
        "blue" => Some(Color::Blue),
        "magenta" => Some(Color::Magenta),
        "cyan" => Some(Color::Cyan),
        "white" => Some(Color::White),
        _ => None,
    }
}

fn color_line(line: &mut String, re: &Regex, color: &Color) {
    let mut offset: usize = 0;
    for c in re.captures_iter(line.clone().as_str()) {
        for m in c.iter() {
            let before = line.len();
            color_match(line, &m.unwrap(), offset, &color);
            let after = line.len();
            assert!(after > before);
            offset += after - before;
        }
    }
}

fn color_match(line: &mut String, mat: &Match, offset: usize, color: &Color) {
    let colored = match color {
        Color::Black => to_black(mat.as_str()),
        Color::Red => to_red(mat.as_str()),
        Color::Green => to_green(mat.as_str()),
        Color::Yellow => to_yellow(mat.as_str()),
        Color::Blue => to_blue(mat.as_str()),
        Color::Magenta => to_magenta(mat.as_str()),
        Color::Cyan => to_cyan(mat.as_str()),
        Color::White => to_white(mat.as_str()),
    };
    let start = mat.start() + offset;
    let end = mat.end() + offset;
    line.replace_range(start..end, &colored);
}

fn to_black(s: &str) -> String {
    format!("\x1b[90m{}\x1b[0m", s)
}
fn to_red(s: &str) -> String {
    format!("\x1b[91m{}\x1b[0m", s)
}
fn to_green(s: &str) -> String {
    format!("\x1b[92m{}\x1b[0m", s)
}
fn to_yellow(s: &str) -> String {
    format!("\x1b[93m{}\x1b[0m", s)
}
fn to_blue(s: &str) -> String {
    format!("\x1b[94m{}\x1b[0m", s)
}
fn to_magenta(s: &str) -> String {
    format!("\x1b[95m{}\x1b[0m", s)
}
fn to_cyan(s: &str) -> String {
    format!("\x1b[96m{}\x1b[0m", s)
}
fn to_white(s: &str) -> String {
    format!("\x1b[97m{}\x1b[0m", s)
}

#[test]
fn test_color_line() {
    let cases = [
        (
            "hello world",
            Regex::new("hello").unwrap(),
            Color::Black,
            "\x1b[90mhello\x1b[0m world",
        ),
        (
            "name",
            Regex::new("name").unwrap(),
            Color::Red,
            "\x1b[91mname\x1b[0m",
        ),
        (
            r#"{"name":"adamlouis","language":"rust"}"#,
            Regex::new("name").unwrap(),
            Color::Green,
            "{\"\x1b[92mname\x1b[0m\":\"adamlouis\",\"language\":\"rust\"}",
        ),
        (
            r#"{"name":"adamlouis","language":"rust"}"#,
            Regex::new(r#""name":"adamlouis""#).unwrap(),
            Color::Yellow,
            "{\x1b[93m\"name\":\"adamlouis\"\x1b[0m,\"language\":\"rust\"}",
        ),
        (
            r#"{"name":"adamlouis","language":"rust"}"#,
            Regex::new(r#""name":"[^"]*""#).unwrap(),
            Color::Blue,
            "{\x1b[94m\"name\":\"adamlouis\"\x1b[0m,\"language\":\"rust\"}",
        ),
        (
            "hello hello world",
            Regex::new("hello").unwrap(),
            Color::Magenta,
            "\x1b[95mhello\x1b[0m \x1b[95mhello\x1b[0m world",
        ),
        (
            "hello world hello",
            Regex::new("hello").unwrap(),
            Color::Cyan,
            "\x1b[96mhello\x1b[0m world \x1b[96mhello\x1b[0m",
        ),
        (
            "hellllllllllllllllllllllllo world",
            Regex::new("hel+o").unwrap(),
            Color::White,
            "\x1b[97mhellllllllllllllllllllllllo\x1b[0m world",
        ),
    ];

    for c in cases.iter() {
        let mut line = c.0.to_string();
        color_line(&mut line, &c.1, &c.2);
        assert_eq!(line, c.3);
    }
}
