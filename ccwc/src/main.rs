use clap::Parser;
use std::{fs, path::PathBuf};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Count characters
    #[arg(short = 'c')]
    characters: bool,

    /// Path to the file
    file: PathBuf,
}

fn main() {
    let args = Args::parse();

    if args.characters {
        count_characters(args.file)
    }

    //println!("Path: {}", args.file.display());
}

fn count_characters(file: PathBuf) {
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
