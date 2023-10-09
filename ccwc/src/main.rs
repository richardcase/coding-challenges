use clap::Parser;
use std::{fs, path::PathBuf};
use std::string;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Count characters
    #[arg(short = 'c', long)]
    bytes: bool,

    /// Count lines
    #[arg(short = 'l', long)]
    lines: bool,

    /// Count words
    #[arg(short = 'w', long)]
    words: bool,

    /// Count characters
    #[arg(short = 'm', long)]
    chars: bool,

    /// Path to the file
    file: PathBuf,
}

fn main() {
    let args = Args::parse();

    // Support the default options if non are supplied
    let default =  !args.chars && !args.words && !args.lines && !args.bytes;

    let filename = match args.file.file_name() {
        None => { panic!("Couldn't get filename")}
        Some(filename) => { filename }
    };

    let mut output = string::String::new();

    if args.lines  || default{
        let lines = count_lines(&args.file);
        output.push_str(lines.to_string().as_str());
        output.push_str(" ");
    }
    if args.words || default {
        let words = count_words(&args.file);
        output.push_str(words.to_string().as_str());
        output.push_str(" ");
    }
    if args.bytes || default {
        let bytes = count_bytes(&args.file);
        output.push_str(bytes.to_string().as_str());
        output.push_str(" ");
    }
    if args.chars {
        let chars = count_characters(&args.file);
        output.push_str(chars.to_string().as_str());
        output.push_str(" ");
    }

    output.push_str(filename.to_str().unwrap());

    println!("{output}");
}

fn count_bytes(file: &PathBuf) -> usize {
    let result = fs::read_to_string(&file);
    
    let contents = match result {
        Ok(contents) => { contents },
        Err(error) => { panic!("Err when opening file {}", error)}
    };

    contents.bytes().len()
}

fn count_lines(file: &PathBuf) -> usize {
    let result = fs::read_to_string(&file);

    let contents = match result {
        Ok(contents) => { contents },
        Err(error) => { panic!("Err when opening file {}", error)}
    };

    let mut num_lines = 0;
    for _line in contents.lines() {
        num_lines += 1;
    }

    num_lines
}

fn count_words(file: &PathBuf) -> usize {
    let result = fs::read_to_string(&file);

    let contents = match result {
        Ok(contents) => { contents },
        Err(error) => { panic!("Err when opening file {}", error)}
    };

    let mut num_words = 0;
    for line in contents.lines() {
        let count = line.split_whitespace().count();
        num_words += count;
    }

    num_words
}

fn count_characters(file: &PathBuf) -> usize {
    let result = fs::read_to_string(&file);

    let contents = match result {
        Ok(contents) => { contents },
        Err(error) => { panic!("Err when opening file {}", error)}
    };

     contents.chars().count()
}
