use clap::Parser;
use std::fs;
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
    file: Option<String>,
}

fn main() {
    let args = Args::parse();

    // Support the default options if non are supplied
    let default =  !args.chars && !args.words && !args.lines && !args.bytes;

    let file:String = match args.file {
        None => { String::new() },
        Some(filename) => { filename }
    };

    let mut contents = String::new();
    if file.is_empty() || file == "-" {
        // TODO: read from stdin
        //panic!("not implemented yet");
        let stdin = std::io::stdin();
        let mut handle = stdin.lock();
        std::io::stdin()
        handle.read_to_string(&mut contents)?;
    } else {
        // Read the contents from a file
        let result = fs::read_to_string(&file);

        contents = match result {
            Ok(contents) => { contents },
            Err(error) => { panic!("Err when opening file {}", error)}
        };
    }

    let mut output = string::String::new();

    if args.lines  || default{
        let lines = count_lines(&contents);
        output.push_str(lines.to_string().as_str());
        output.push_str(" ");
    }
    if args.words || default {
        let words = count_words(&contents);
        output.push_str(words.to_string().as_str());
        output.push_str(" ");
    }
    if args.bytes || default {
        let bytes = count_bytes(&contents);
        output.push_str(bytes.to_string().as_str());
        output.push_str(" ");
    }
    if args.chars {
        let chars = count_characters(&contents);
        output.push_str(chars.to_string().as_str());
        output.push_str(" ");
    }

    output.push_str(file.as_str());

    println!("{output}");
}

fn count_bytes(contents: &str) -> usize {
    contents.bytes().len()
}

fn count_lines(contents: &str) -> usize {
    let mut num_lines = 0;
    for _line in contents.lines() {
        num_lines += 1;
    }

    num_lines
}

fn count_words(contents: &str) -> usize {
    let mut num_words = 0;
    for line in contents.lines() {
        let count = line.split_whitespace().count();
        num_words += count;
    }

    num_words
}

fn count_characters(contents: &str) -> usize {
     contents.chars().count()
}
