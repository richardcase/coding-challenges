use clap::Parser;
use std::{fs, path::PathBuf};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Count characters
    #[arg(short = 'c')]
    characters: bool,

    /// Count lines
    #[arg(short = 'l')]
    lines: bool,

    /// Count words
    #[arg(short = 'w')]
    words: bool,

    /// Path to the file
    file: PathBuf,
}

fn main() {
    let args = Args::parse();

    if args.characters {
        count_characters(&args.file)
    }
    if args.lines {
        count_lines(&args.file)
    }
    if args.words {
        count_words(&args.file)
    }
}

fn count_characters(file: &PathBuf) {
    let result = fs::read_to_string(&file);
    
    let contents = match result {
        Ok(contents) => { contents },
        Err(error) => { panic!("Err when opening file {}", error)}
    };

    let num_bytes = contents.bytes().len();

    let filename = match file.file_name() {
        None => { panic!("Couldn't get filename")}
        Some(filename) => { filename }
    };

    println!("{} {}", num_bytes, filename.to_str().unwrap());
}

fn count_lines(file: &PathBuf) {
    let result = fs::read_to_string(&file);

    let contents = match result {
        Ok(contents) => { contents },
        Err(error) => { panic!("Err when opening file {}", error)}
    };

    let mut num_lines = 0;
    for _line in contents.lines() {
        num_lines += 1;
    }

    let filename = match file.file_name() {
        None => { panic!("Couldn't get filename")}
        Some(filename) => { filename }
    };

    println!("{} {}", num_lines, filename.to_str().unwrap());
}

fn count_words(file: &PathBuf) {
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

    let filename = match file.file_name() {
        None => { panic!("Couldn't get filename")}
        Some(filename) => { filename }
    };

    println!("{} {}", num_words, filename.to_str().unwrap());
}
