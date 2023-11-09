use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use unidecode::unidecode;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: memento_search <file_path> <keyword1> <keyword2> ... <keywordN>");
        std::process::exit(1);
    }

    let file_path = &args[1];
    let keywords = &args[2..];

    // Colores ANSI
    let red_start = "\x1b[31m";
    let color_reset = "\x1b[0m";

    // Abrir el archivo
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    // Iterar sobre las líneas del archivo
    for line in reader.lines() {
        let line = line?;
        let normalized_line = unidecode(&line).to_lowercase();

        // Verificar si todas las palabras clave normalizadas están en la línea normalizada
        if keywords.iter().all(|k| normalized_line.contains(&unidecode(&k.to_lowercase()))) {
            let mut highlighted_line = line.clone();
            for keyword in keywords {
                highlighted_line = highlighted_line.replace(keyword, &format!("{}{}{}", red_start, keyword, color_reset).as_str());
            }
            println!("{}", highlighted_line);
        }
    }

    Ok(())
}

