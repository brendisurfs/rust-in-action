/*
  Main focus: Arrays, Slices, And Vecs. Mostly on Vecs, they are the most flexible. I am no leet programmer (yet).
OLD  @dev: problem: we want to store n lines of context aroudn a match.
  we will collect lines within n that match the tags we store.
*/
use clap::{App, Arg};
use regex::Regex;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    println!("\nGREP-LITE\n");

    // instanciate a new App from clap.
    let args = App::new("grep-lite")
        .version("0.1")
        .about("searches for pattern")
        .arg(
            Arg::new("pattern")
                .help("the pattern to seach for")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::new("input")
                .help("file to search")
                .takes_value(true)
                .required(true),
        )
        .get_matches();

    // pass our cli argument into the the regex pattern.
    let pattern = args.value_of("pattern").unwrap();
    let re = Regex::new(pattern).unwrap();

    let input = args.value_of("input").unwrap_or("-");

    if input == "-" {
        let stdin = io::stdin();
        let reader = stdin.lock();
        process_lines(reader, re);
    } else {
        let f = File::open(input).unwrap();
        let reader = BufReader::new(f);
        process_lines(reader, re);
    }
}

fn process_lines<T: BufRead + Sized>(reader: T, re: Regex) {
    for line_ in reader.lines() {
        let line = line_.unwrap();
        match re.find(&line) {
            Some(_) => println!("{line}"),
            None => (),
        }
    }
}
