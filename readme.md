# wordfreq

A simple command-line utilitiy for processing text, i. e. for counting and normalizing words.

## Features

- **Current Features**: Counts all words (currently, only all the words of a text file are returned, not the individual frequencies), normalizing words containing, for example, umlauts and special characters. Checks whether the input text file is UTF-8 encoded; if not, it assumes Windows-1252 encoding.
- **Future features**: Flag for displaying each word count in the output, flag for not-normalizing words with special characters.

## Installation

### Prerequisites

- Rust 1.70+ (install from [rustup.rs](https://rustup.rs/))

### Build from Source

```bash
git clone https://github.com/<yourusername>/textcmd.git
cd textcmd
cargo build --release
```

The compiled binary will be available at `./target/release/textcmd`

## Usage

### Basic Usage

```bash
cargo run -- <filename>
```

### Examples

```bash
# Determines all words in an input text file and writes the results to the standard output file "mydic_curr.txt".
cargo run -- -f document.txt

# Using the compiled binary
./target/release/wordfreq -f document.txt
```

### Sample Output

[Coming soon.]

## Project Structure

```
textcmd/
├── Cargo.toml           # Project manifest
├── Cargo.lock           # Dependency lock file
├── readme.md            # This file
└── src/
    └── wordfreq.rs      # Main application code
```

## Error Handling

The program is currently returning the following error messages:

### No filename provided

```
$ cargo run
error: the following required arguments were not provided:
  --file <FILE>

Usage: wordfreq.exe --file <FILE>

For more information, try '--help'.
```

### File doesn't exist

```
$ cargo run -- -f notexistingfile.txt
error: invalid value 'notexistingfile.txt' for '--file <FILE>': file does not exist: notexistingfile.txt

For more information, try '--help'.
```

### Permission denied

```
$ cargo run -- /etc/notenoughrights.txt
error: invalid value 'C:/msys64/etc/notenoughrights.txt' for '--file <FILE>': cannot open file: Zugriff verweigert (os error 5)

For more information, try '--help'.
```

## Technical Details

- **Language**: Rust (Edition 2024)
- **Dependencies**: None (uses only standard library)
- **Minimum Rust Version**: 1.70.0
- **Platform**: Cross-platform (Linux, macOS, Windows)

## License

MIT License - See LICENSE file for details
