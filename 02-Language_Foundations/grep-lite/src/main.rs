use clap::{Arg, Command};
use regex::Regex;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;

fn process_lines<T: BufRead + Sized>(reader: T, re: Regex) {
    for line_ in reader.lines() {
        let line = line_.unwrap();
        match re.find(&line) {
            Some(_) => println!("{}", line),
            None => (),
        }
    }
}

fn main() {
    let matches = Command::new("grep-lite")
        .version("0.1")
        .author("Todd Leonhardt")
        .about("searches for patterns")
        .arg(
            Arg::new("pattern")
                .help("The pattern to search for")
                .required(true),
        )
        .arg(Arg::new("input").help("File to search").required(false))
        .get_matches();

    let pattern = matches.get_one::<String>("pattern").expect("required");
    let re = Regex::new(pattern).unwrap();

    let default_input = "-".to_string();

    let input = matches.get_one::<String>("input").unwrap_or(&default_input);

    if input == &default_input {
        let stdin = io::stdin();
        let reader = stdin.lock();
        process_lines(reader, re);
    } else {
        let f = File::open(input).unwrap();
        let reader = BufReader::new(f);
        process_lines(reader, re);
    }
}
