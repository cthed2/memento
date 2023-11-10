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
    let keywords = &args[2..].iter().map(|k| unidecode(k).to_lowercase()).collect::<Vec<String>>();
    let keywords_regex = build_keywords_regex(keywords).map_err(|e| {
        eprintln!("Error building regex: {}", e);
        std::io::Error::new(std::io::ErrorKind::Other, "Regex build error")
    })?;

    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        let normalized_line = unidecode(&line).to_lowercase();
        if keywords.iter().all(|keyword| normalized_line.contains(keyword)) {
            println!("{}", highlight_keywords(&line, &keywords_regex));
        }
    }

    Ok(())
}

fn build_keywords_regex(keywords: &[String]) -> Result<Regex, regex::Error> {
    let pattern: String = keywords
        .iter()
        .map(|k| regex::escape(k))
        .collect::<Vec<String>>()
        .join("|");
    Regex::new(&pattern)
}

fn highlight_keywords(line: &str, keywords_regex: &Regex) -> String {
    keywords_regex.replace_all(line, |caps: &regex::Captures| {
        Colour::Red.paint(&caps[0]).to_string()
    }).to_string()
}

