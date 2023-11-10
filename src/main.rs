use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use regex::Regex; 
use ansi_term::Colour;
use unidecode::unidecode;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: memento <file_path> <keyword1> <keyword2> ... <keywordN>");
        std::process::exit(1);
    }

    let file_path = &args[1];
    let keywords = &args[2..];
    let keywords_regex = build_keywords_regex(keywords).map_err(|e| {
        eprintln!("Error building regex: {}", e);
        std::io::Error::new(std::io::ErrorKind::Other, "Regex build error")
    })?;

    let file = File::open(file_path).map_err(|error| {
        eprintln!("Error opening file: {}", error);
        std::io::Error::new(std::io::ErrorKind::Other, "File open error")
    })?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        print_highlighted_line(&line, &keywords_regex);
    }

    Ok(())
}

fn build_keywords_regex(keywords: &[String]) -> Result<Regex, regex::Error> {
    let pattern: String = keywords
        .iter()
        .map(|k| regex::escape(&unidecode(&k.to_lowercase())))
        .collect::<Vec<String>>()
        .join("|");
    Regex::new(&pattern)
}

fn print_highlighted_line(line: &str, keywords_regex: &Regex) {
    let highlighted_line = keywords_regex.replace_all(line, |caps: &regex::Captures| {
        Colour::Red.paint(&caps[0]).to_string()
    });
    println!("{}", highlighted_line);
}

