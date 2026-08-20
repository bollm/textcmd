use clap::Parser;
use clap::Subcommand;
use encoding_rs::WINDOWS_1252;
use encoding_rs_io::DecodeReaderBytesBuilder;
use std::collections::HashMap;
use std::error::Error;
use std::ffi::OsStr;
use std::fs;
use std::fs::File;
use std::io::Read;
use std::io::{BufRead, BufReader, Write};
use std::path::PathBuf;
use unicode_normalization::UnicodeNormalization;

/**
 * A. Bollmann (Vers. 0.1: ChatGPT, d.h. wesentliche Wandlung Java -> Rust)
 *
 * Vers. 0.2 - 17.08.2026:
 *             Neue Funktion "is_utf8" u. andere Dinge
 * Vers. 0.3 - 18.08.2026:
 *             Funktionalität auf Clap umgestellt und erweitert
 */

/// Standard-Ausgabedatei
const OUT_FILE_DEF: &str = "mydic_curr.txt";

const PRG_VERSION: &str = env!("CARGO_PKG_VERSION");

const CUSTOM_HELP: &str = r#"wordfreq
A program that determines the word frequencies in an input text file
and normalizes words containing, for example, umlauts and special characters.

USAGE:
    wordfreq.exe --file <FILE> --out_file <OUT_FILE> <COMMAND>

SUBCOMMANDS:
    addhelp  Useful for printing out additional help information
    help     Print this message or the help of the given subcommand(s)
"#;

/// A program for determining word frequencies in an input text

#[derive(Parser)]
#[command(
    name = "wordfreq",
    version = PRG_VERSION,
    about = "A program for determining word frequencies in an input text. It also normalizes the detected words.",
    long_about = "This program determines all the words in a passed input text and normalizes the detected words.\n 
(Currently, only the words are returned, not the individual frequencies)." // Output when the --help option has been entered.
)]

struct Cli {
    /// A required input file path
    #[arg(
        short = 'f',
        long = "file",
        help = "Path to the text file",
        required(true),
        value_parser = parse_existing_file,
    )]
    file: PathBuf,
    // Keinesfalls (s. nä. Zeile) verwenden bei file mit PathBuf:
    // ->    file: String,
    /// Print word frequencies into custom file path (path name is <NAME>
    #[arg(
        short = 'o',
        long = "out_file",
        default_value_os_t = PathBuf::from(OUT_FILE_DEF)
        //value_parser(count_in_range)
    )]
    out_file: PathBuf,
    /// Enable verbose mode
    #[arg(short, long)]
    verbose: bool,
    /// Optional subcommand
    #[command(subcommand)]
    command: Option<Commands>,
    //command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Useful for printing out additional help information
    Addhelp {
        //#[arg(short, long],
        //addhelp: String,
    },
}

pub type Result<T, E = dyn Error> = std::result::Result<T, E>;

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    //let args: Cli = Cli::parse();
    let cli = Cli::parse();

    if cli.verbose {
        println!("Path of input file name: {}", cli.file.display());
    }
    let output_file_name = cli.out_file.display();

    println!("Input file : {}", cli.file.display());
    println!("Output file: {}", output_file_name);

    match cli.command {
        Some(Commands::Addhelp {}) => {
            println!();
            println!("Additional help information about this program ... (see below):");
            println!("{}", CUSTOM_HELP.to_string())
        }

        None => {
            println!();
            println!("No subcommand specified.");
        }
    }

    let mut use_other_encoding = false;
    //
    let file_name = &cli.file.display();

    if is_utf8(&file_name.to_string()) {
        println!("Input file \"{}\" is UTF-8 encoded.", file_name);
    } else {
        println!("Input file \"{}\" is NOT UTF-8 encoded.", file_name);
        if cli.verbose {
            println!(
                "Attempting to process input file \"{}\" with Windows-1252 encoding ...",
                file_name
            );
        }
        use_other_encoding = true;
    }
    //
    let mut freq = HashMap::<String, u64>::new();
    let mut entries: Vec<(String, u64)>;
    //

    if !use_other_encoding {
        let file = File::open(&file_name.to_string())?;

        let reader = BufReader::new(file);

        //let mut freq = HashMap::<String, u64>::new();

        for line in reader.lines() {
            let line = line?;

            for token in tokenize(&line) {
                if token.len() > 1 {
                    *freq.entry(token).or_insert(0) += 1;
                }
            }
        }

        // Sortierung wie im Java-Code:
        // 1. nach Häufigkeit
        // 2. lexikographisch
        //let mut entries: Vec<(String, u64)> = freq.into_iter().collect();
        entries = freq.into_iter().collect();
        //
        entries.sort_by(|a, b| {
            a.1.cmp(&b.1) // Frequenz
                .then_with(|| a.0.cmp(&b.0)) // Wort
        });

        // Ausgabe (nur Wörter, wie im Java-Code)
        let mut out = File::create(output_file_name.to_string())?;
        let mut word_count = 0;
        for (word, _) in &entries {
            writeln!(out, "{word}")?;
            word_count += 1;
        }
        println!(
            "The output was written to the file  \"{}\".",
            output_file_name
        );
        if cli.verbose {
            println!("Number of determined words in input file: {}", word_count);
        }
        //Ok(())
    } else {
        let file = File::open(&file_name.to_string())?;
        //let mut reader = BufReader::new(
        let reader = BufReader::new(
            DecodeReaderBytesBuilder::new()
                .encoding(Some(WINDOWS_1252))
                .build(file),
        );
        for line in reader.lines() {
            let line = line?;
            for token in tokenize(&line) {
                if token.len() > 1 {
                    *freq.entry(token).or_insert(0) += 1;
                }
            }
        }
        // Sortierung wie im Java-Code:
        // 1. nach Häufigkeit
        // 2. lexikographisch
        entries = freq.into_iter().collect();
        //
        entries.sort_by(|a, b| {
            a.1.cmp(&b.1) // Frequenz
                .then_with(|| a.0.cmp(&b.0)) // Wort
        });

        // Ausgabe (nur Wörter, wie im Java-Code)
        let mut out = File::create(output_file_name.to_string())?;
        let mut word_count = 0;
        for (word, _) in &entries {
            writeln!(out, "{word}")?;
            word_count += 1;
        }
        println!(
            "The output was written to the file  \"{}\".",
            output_file_name
        );
        if cli.verbose {
            println!("Number of determined words in input file: {}", word_count);
        }
    }
    Ok(())
}

/// Check whether the specified file exists
fn parse_existing_file(s: &str) -> Result<PathBuf, String> {
    if s.is_empty() {
        return Err("file name must not be empty".into());
    }

    // sehr grobe, aber portable Plausibilitätsprüfung
    if s.contains('\0') {
        return Err("file name contains NUL byte".into());
    }

    let path = PathBuf::from(s);

    // optional: letzte Pfadkomponente prüfen
    if let Some(name) = path.file_name() {
        if name == OsStr::new("") {
            return Err("invalid file name".into());
        }
    }

    if !path.exists() {
        return Err(format!("file does not exist: {}", path.display()));
    }

    // --- NEU: reguläre Datei erzwingen ---
    let meta = path
        .metadata()
        .map_err(|e| format!("cannot access file metadata: {e}"))?;

    if !meta.is_file() {
        return Err(format!("not a regular file: {}", path.display()));
    }

    // --- NEU: Textdatei-Plausibilitätsprüfung ---
    const MAX_CHECK_BYTES: usize = 8 * 1024;

    let mut file = File::open(&path).map_err(|e| format!("cannot open file: {e}"))?;

    let mut buffer = [0u8; MAX_CHECK_BYTES];
    let n = file
        .read(&mut buffer)
        .map_err(|e| format!("cannot read file: {e}"))?;

    let slice = &buffer[..n];

    // starkes Binärsignal: NUL-Byte
    if slice.iter().any(|&b| b == 0) {
        return Err(format!(
            "file does not look like a text file (contains NUL byte): {}",
            path.display()
        ));
    }

    // UTF-8-Plausibilitätsprüfung
    if std::str::from_utf8(slice).is_err() {
        return Err(format!(
            "file does not look like a UTF-8 text file: {}",
            path.display()
        ));
    }

    Ok(path)
}

/// Tokenisierung ähnlich StreamTokenizer (Java-Code):
/// → Nur relevante Zeichen behalten
/// → Split nach Nicht-Wortzeichen
fn tokenize(line: &str) -> Vec<String> {
    line.split(|c: char| !is_word_char(c))
        .filter_map(|raw| {
            let norm = normalize(raw);
            if norm.is_empty() { None } else { Some(norm) }
        })
        .collect()
}

/// Definiert, welche Zeichen als "Wortzeichen" gelten
/// (ähnlich wordChars im Java-Code)
fn is_word_char(c: char) -> bool {
    c.is_ascii_alphabetic()
        || matches!(
            c,
            'ä' | 'ö'
                | 'ü'
                | 'Ä'
                | 'Ö'
                | 'Ü'
                | 'ß'
                | 'á'
                | 'à'
                | 'â'
                | 'é'
                | 'è'
                | 'ê'
                | 'ó'
                | 'ò'
                | 'ô'
                | 'ú'
                | 'ù'
                | 'û'
        )
}

/// Zentrale Normalisierung:
/// - Umlaute → ae/oe/ue
/// - ß → ss
/// - diakritische Zeichen → ASCII
/// - Sonderzeichen entfernen
/// - nicht-lateinische Zeichen ignorieren
fn normalize(input: &str) -> String {
    let mut s = input.to_string();

    // --- 1. Explizite Ersetzungen (wie im Java-Code) ---
    s = s
        .replace("ä", "ae")
        .replace("ö", "oe")
        .replace("ü", "ue")
        .replace("Ä", "Ae")
        .replace("Ö", "Oe")
        .replace("Ü", "Ue")
        .replace("ß", "ss");

    // --- 2. Unicode-Normalisierung (entfernt diakritische Zeichen) ---
    // z.B. é -> e oder à -> a
    let s: String = s
        .nfd() // decomposed form
        .filter(|c| !is_combining_mark(*c))
        .collect();

    // --- 3. Nur ASCII-Buchstaben behalten ---
    let s: String = s.chars().filter(|c| c.is_ascii_alphabetic()).collect();

    // KEIN Lowercasing mehr!
    //s.to_lowercase()
    s
}

/// Prüft, ob ein Zeichen ein diakritisches Zeichen ist
fn is_combining_mark(c: char) -> bool {
    use unicode_normalization::char::is_combining_mark;
    is_combining_mark(c)
}

/// Prüft, ob Datei mit übergeb. Path UTF-8-codiert ist
fn is_utf8(path: &str) -> bool {
    let bytes = fs::read(path).expect("Unable to read file");
    std::str::from_utf8(&bytes).is_ok()
}
