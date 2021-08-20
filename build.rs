//! Generate the `words.rs` file.
use std::env;
use std::fs;
use std::path::Path;

fn load_words() -> Vec<String> {
    let wordspath = Path::new("./words.txt");
    let words = fs::read_to_string(wordspath).unwrap();
    words
        .split_whitespace()
        .map(|s| s.trim().to_owned())
        .collect()
}

fn format_wordsfile(words: &[String]) -> String {
    let wordcount = words.len();
    let wordline = words
        .iter()
        .map(|w| format!("\"{}\"", w))
        .collect::<Vec<String>>()
        .join(", ");
    format!(
        "pub const WORDS: [&str; {}] = [{}];",
        wordcount, wordline
    )
}

fn write_wordsfile(content: String) {
    let outdir = env::var_os("OUT_DIR").unwrap();
    let path = Path::new(&outdir).join("words.rs");
    fs::write(&path, content).unwrap();
}

fn main() {
    let words = load_words();
    let content = format_wordsfile(&words);
    dbg!("aah");
    write_wordsfile(content);
}
