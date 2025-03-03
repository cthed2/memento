use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;
use regex::Regex;
use ansi_term::Colour::{Red, Green, Yellow, Blue, Purple};
use ansi_term::Style;
use unidecode::unidecode;
use clap::{App, Arg};

struct SearchOptions {
    file_path: String,
    keywords: Vec<String>,
    case_sensitive: bool,
    use_regex: bool,
    line_numbers: bool,
    context_lines: usize,
    match_any: bool,
    color_mode: String,
    fuzzy_search: bool,
    fuzzy_threshold: usize,
}

fn main() -> io::Result<()> {
    let matches = App::new("memento")
        .version("1.2.0")
        .about("Search for keywords in text files with highlighting")
        .arg(Arg::with_name("file")
            .help("File to search")
            .required(true)
            .index(1))
        .arg(Arg::with_name("keywords")
            .help("Keywords to search for")
            .required(true)
            .multiple(true)
            .index(2))
        .arg(Arg::with_name("case_sensitive")
            .short("c")
            .long("case-sensitive")
            .help("Perform case-sensitive search"))
        .arg(Arg::with_name("regex")
            .short("r")
            .long("regex")
            .help("Treat keywords as regex patterns"))
        .arg(Arg::with_name("line_numbers")
            .short("n")
            .long("line-numbers")
            .help("Show line numbers"))
        .arg(Arg::with_name("context")
            .short("C")
            .long("context")
            .takes_value(true)
            .help("Show N lines of context before and after matches"))
        .arg(Arg::with_name("or")
            .short("o")
            .long("or")
            .help("Match any keyword (OR logic) instead of all keywords (AND logic)"))
        .arg(Arg::with_name("color")
            .long("color")
            .takes_value(true)
            .default_value("auto")
            .possible_values(&["always", "auto", "never", "multi"])
            .help("Colorization mode"))
        .arg(Arg::with_name("fuzzy")
            .short("f")
            .long("fuzzy")
            .help("Enable fuzzy search for approximate matches"))
        .arg(Arg::with_name("threshold")
            .long("threshold")
            .takes_value(true)
            .default_value("2")
            .help("Maximum edit distance for fuzzy matching (default: 2)"))
        .get_matches();

    let options = SearchOptions {
        file_path: matches.value_of("file").unwrap().to_string(),
        keywords: matches.values_of("keywords").unwrap().map(String::from).collect(),
        case_sensitive: matches.is_present("case_sensitive"),
        use_regex: matches.is_present("regex"),
        line_numbers: matches.is_present("line_numbers"),
        context_lines: matches.value_of("context").unwrap_or("0").parse().unwrap_or(0),
        match_any: matches.is_present("or"),
        color_mode: matches.value_of("color").unwrap_or("auto").to_string(),
        fuzzy_search: matches.is_present("fuzzy"),
        fuzzy_threshold: matches.value_of("threshold").unwrap_or("2").parse().unwrap_or(2),
    };

    search_file(&options)
}

fn search_file(options: &SearchOptions) -> io::Result<()> {
    let file_path = Path::new(&options.file_path);
    let file = match File::open(file_path) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Error opening file '{}': {}", options.file_path, e);
            return Err(e);
        }
    };

    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().collect::<Result<_, _>>()?;

    // Build regex patterns (for non-fuzzy search)
    let keyword_regexes = if !options.fuzzy_search && options.use_regex {
        build_regex_patterns(&options.keywords, options.case_sensitive)?
    } else if !options.fuzzy_search {
        build_literal_patterns(&options.keywords, options.case_sensitive)?
    } else {
        Vec::new() // Empty for fuzzy search mode
    };

    // Store matched line indices and their matched words
    let mut matches_info: Vec<(usize, Vec<String>)> = Vec::new();

    // Find matching lines
    for (i, line) in lines.iter().enumerate() {
        let normalized_line = if options.case_sensitive {
            line.clone()
        } else {
            unidecode(line).to_lowercase()
        };
        
        let mut matched_terms = Vec::new();
        
        if options.fuzzy_search {
            // Split the line into words for fuzzy matching
            let words: Vec<&str> = normalized_line.split_whitespace().collect();
            
            for keyword in &options.keywords {
                let normalized_keyword = if options.case_sensitive {
                    keyword.clone()
                } else {
                    unidecode(keyword).to_lowercase()
                };
                
                // Check for fuzzy matches in the line
                let mut found_fuzzy_match = false;
                
                for word in &words {
                    if levenshtein_distance(word, &normalized_keyword) <= options.fuzzy_threshold {
                        matched_terms.push(word.to_string());
                        found_fuzzy_match = true;
                        break;
                    }
                }
                
                // If using AND logic and any keyword doesn't match, skip this line
                if !found_fuzzy_match && !options.match_any {
                    matched_terms.clear();
                    break;
                }
            }
            
            // If we have matches or using OR logic with at least one match
            if (!matched_terms.is_empty() && options.match_any) || 
               (matched_terms.len() == options.keywords.len() && !options.match_any) {
                matches_info.push((i, matched_terms));
            }
        } else {
            // Standard non-fuzzy matching
            let matches_all = options.keywords.iter().all(|keyword| {
                let normalized_keyword = if options.case_sensitive {
                    keyword.clone()
                } else {
                    unidecode(keyword).to_lowercase()
                };
                normalized_line.contains(&normalized_keyword)
            });

            let matches_any = options.keywords.iter().any(|keyword| {
                let normalized_keyword = if options.case_sensitive {
                    keyword.clone()
                } else {
                    unidecode(keyword).to_lowercase()
                };
                normalized_line.contains(&normalized_keyword)
            });

            if (options.match_any && matches_any) || (!options.match_any && matches_all) {
                // For non-fuzzy search, we don't track what matched specifically
                matches_info.push((i, Vec::new()));
            }
        }
    }

    // Extract just the indices for convenience
    let matched_indices: Vec<usize> = matches_info.iter().map(|(idx, _)| *idx).collect();

    // Print results with context
    let mut already_printed = std::collections::HashSet::new();
    for (idx, (line_idx, matched_terms)) in matches_info.iter().enumerate() {
        let i = *line_idx;
        let start_idx = if i >= options.context_lines { i - options.context_lines } else { 0 };
        let end_idx = std::cmp::min(i + options.context_lines + 1, lines.len());

        for j in start_idx..end_idx {
            if already_printed.contains(&j) {
                continue;
            }
            already_printed.insert(j);

            let is_match_line = j == i;
            let line_prefix = if options.line_numbers {
                format!("{}: ", j + 1)
            } else {
                String::new()
            };

            if is_match_line {
                let highlighted = if options.color_mode == "never" {
                    lines[j].clone()
                } else if options.fuzzy_search {
                    // For fuzzy search, highlight the matched terms
                    highlight_fuzzy_matches(&lines[j], matched_terms, options.case_sensitive)
                } else if options.color_mode == "multi" {
                    highlight_keywords_multi(&lines[j], &keyword_regexes)
                } else {
                    highlight_keywords(&lines[j], &keyword_regexes)
                };

                if options.line_numbers {
                    println!("{}{}", Green.paint(line_prefix), highlighted);
                } else {
                    println!("{}", highlighted);
                }
            } else {
                // Context line
                if options.line_numbers {
                    println!("{}{}", Style::new().dimmed().paint(line_prefix), Style::new().dimmed().paint(&lines[j]));
                } else {
                    println!("{}", Style::new().dimmed().paint(&lines[j]));
                }
            }
        }

        // Add separator between context blocks
        if options.context_lines > 0 && idx < matches_info.len() - 1 {
            println!("--");
        }
    }

    if matched_indices.is_empty() {
        println!("No matches found.");
    } else {
        println!("\nFound {} matching lines", matched_indices.len());
    }

    Ok(())
}

// Implement Levenshtein distance for fuzzy matching
fn levenshtein_distance(s1: &str, s2: &str) -> usize {
    let len1 = s1.chars().count();
    let len2 = s2.chars().count();
    
    if len1 == 0 { return len2; }
    if len2 == 0 { return len1; }
    
    let s1_chars: Vec<char> = s1.chars().collect();
    let s2_chars: Vec<char> = s2.chars().collect();
    
    let mut matrix = vec![vec![0; len2 + 1]; len1 + 1];
    
    for i in 0..=len1 {
        matrix[i][0] = i;
    }
    
    for j in 0..=len2 {
        matrix[0][j] = j;
    }
    
    for i in 1..=len1 {
        for j in 1..=len2 {
            let cost = if s1_chars[i-1] == s2_chars[j-1] { 0 } else { 1 };
            
            matrix[i][j] = std::cmp::min(
                std::cmp::min(
                    matrix[i-1][j] + 1,      // deletion
                    matrix[i][j-1] + 1       // insertion
                ),
                matrix[i-1][j-1] + cost      // substitution
            );
        }
    }
    
    matrix[len1][len2]
}

fn highlight_fuzzy_matches(line: &str, matched_terms: &[String], case_sensitive: bool) -> String {
    if matched_terms.is_empty() {
        return line.to_string();
    }
    
    let words: Vec<&str> = line.split_whitespace().collect();
    let mut result = String::new();
    
    for (i, word) in words.iter().enumerate() {
        let normalized_word = if case_sensitive {
            word.to_string()
        } else {
            unidecode(word).to_lowercase()
        };
        
        let mut is_match = false;
        for term in matched_terms {
            if levenshtein_distance(&normalized_word, term) <= 2 {
                is_match = true;
                break;
            }
        }
        
        if is_match {
            result.push_str(&Red.bold().paint(*word).to_string());
        } else {
            result.push_str(word);
        }
        
        if i < words.len() - 1 {
            result.push(' ');
        }
    }
    
    result
}

fn build_regex_patterns(patterns: &[String], case_sensitive: bool) -> io::Result<Vec<Regex>> {
    let mut result = Vec::new();
    for pattern in patterns {
        let regex_str = if case_sensitive {
            pattern.clone()
        } else {
            format!("(?i){}", pattern)
        };
        
        match Regex::new(&regex_str) {
            Ok(regex) => result.push(regex),
            Err(e) => {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    format!("Invalid regex pattern '{}': {}", pattern, e),
                ));
            }
        }
    }
    Ok(result)
}

fn build_literal_patterns(keywords: &[String], case_sensitive: bool) -> io::Result<Vec<Regex>> {
    let mut result = Vec::new();
    for keyword in keywords {
        let regex_str = if case_sensitive {
            regex::escape(keyword)
        } else {
            format!("(?i){}", regex::escape(keyword))
        };
        
        match Regex::new(&regex_str) {
            Ok(regex) => result.push(regex),
            Err(e) => {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    format!("Error building regex for '{}': {}", keyword, e),
                ));
            }
        }
    }
    Ok(result)
}

fn highlight_keywords(line: &str, patterns: &[Regex]) -> String {
    let mut result = line.to_string();
    for pattern in patterns {
        result = pattern.replace_all(&result, |caps: &regex::Captures| {
            Red.bold().paint(&caps[0]).to_string()
        }).to_string();
    }
    result
}

fn highlight_keywords_multi(line: &str, patterns: &[Regex]) -> String {
    let mut result = line.to_string();
    let colors = [Red, Green, Yellow, Blue, Purple];
    
    for (i, pattern) in patterns.iter().enumerate() {
        let color_index = i % colors.len();
        result = pattern.replace_all(&result, |caps: &regex::Captures| {
            colors[color_index].bold().paint(&caps[0]).to_string()
        }).to_string();
    }
    result
}
