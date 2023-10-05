use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Count characters
    #[arg(short)]
    characters: bool,

    /// Path to the file
    file: std::path::PathBuf,
}

fn main() {
    let args = Args::parse();

    if args.characters {
        count_characters(args.file)
    }

    //println!("Path: {}", args.file.display());
}

fn count_characters(file: std::path::PathBuf) {
    let contents = std::fs::read_to_string(&file).expect("could not read file");
    for line in contents.lines() {}

    let filename = file.as_ref().and_then(|name| name.file_name()).unwrap();

    let numbytes = contents.bytes().len();
    println!("{} {:?}", numbytes, file.file_name().to_str().unwrap())
}
