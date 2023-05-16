use clap::{Arg, Command};
use regex::Regex;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

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
        .arg(Arg::new("input").help("File to search").required(true))
        .get_matches();

    let pattern = matches.get_one::<String>("pattern").expect("required");
    let re = Regex::new(pattern).unwrap();

    let input = matches.get_one::<String>("input").expect("required");
    let f = File::open(input).unwrap();
    let reader = BufReader::new(f);

    for line_ in reader.lines() {
        let line = line_.unwrap();

        match re.find(&line) {
            Some(_) => println!("{}", line),
            None => (),
        }
    }
}
