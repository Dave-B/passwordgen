use clap::Parser;
use rand::Rng;
use std::{
    fs::File,
    io::{BufRead, BufReader, Write},
    path::PathBuf,
};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// File in which to save the generated passwords
    #[arg(short, long, default_value = "passwords.txt")]
    output_file: PathBuf,

    /// File from which to load words
    // #[arg(short, long, default_value = "assets/wordlist.txt")]
    #[arg(short, long)]
    wordlist_file: Option<PathBuf>,

    /// Number of passwords to generate
    #[arg(short, long, default_value_t = 250)]
    number_passwords: u16,

    /// Number of words in each password
    #[arg(short, long, default_value_t = 4)]
    password_word_count: u8,

    /// Separator between words in each password
    #[arg(short, long, default_value = "-")]
    separator: String,

    /// Minimum length of words used
    #[arg(long, default_value_t = 3)]
    word_min_length: u8,

    /// Maximum length of words usedline
    #[arg(long, default_value_t = 6)]
    word_max_length: u8,
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();
    let mut wordlist:Vec<String>= Vec::new();

    if args.wordlist_file.is_some() {
        // Load from arg.
        let listfile = args.wordlist_file.unwrap();
        println!("Using specified word list: {}", listfile.display());
        wordlist = get_word_list(listfile);
    } else {
        // Load from asset file included into binary.
        println!("Using internal word list.");
        let wordlist_str = include_str!("assets/wordlist.txt");
        for line in wordlist_str.lines() {
            wordlist.push(line.to_string());
        }

    }

    let mut passwords:Vec<String> = Vec::new();
    for _ in 0..args.number_passwords {
        passwords.push(generate_password(
            &wordlist,
            args.password_word_count,
            &args.separator,
        ));
    }

    let mut output_file = File::create(&args.output_file)?;
    for line in passwords {
        write!(output_file, "{}\n", line)?;
    }
    println!(
        "Generated {} passwords, saved to the file \"{}\".",
        args.number_passwords,
        &args.output_file.to_string_lossy()
    );

    Ok(())
}

/// Load list of words from text file
fn get_word_list(filename: PathBuf) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn generate_password(wordlist: &[String], words_in_password: u8, separator: &String) -> String {
    let wordlist_length = wordlist.len();
    // println!("{}", &wordlist_length);

    let mut rng = rand::thread_rng();
    let mut words: Vec<String> = vec![];
    for _ in 0..words_in_password {
        words.push(wordlist[rng.gen_range(0..wordlist_length)].clone());
    }

    words.join(separator)
}
