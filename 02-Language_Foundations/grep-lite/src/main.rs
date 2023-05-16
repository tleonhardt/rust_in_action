use clap::{Arg, Command};
use regex::Regex;

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
        .get_matches();

    let pattern = matches.get_one::<String>("pattern").expect("required");
    let re = Regex::new(pattern).unwrap();

    let quote = "Every face, every shop, bedroom window, public-house, and
    dark square is a picture feverishly turned--in search of what?
    It is the same with books. What do we seek through millions of pages?";

    for line in quote.lines() {
        match re.find(line) {
            Some(_) => println!("{}", line),
            None => (),
        }
    }
}
