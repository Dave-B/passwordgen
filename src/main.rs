use clap::Parser;
use std::{
    fs::{File},
    io::{BufRead, BufReader},
    path::PathBuf
};
use rand::Rng;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// File in which to save the generated passwords
    #[arg(short, long, default_value = "output.txt")]
    output_file: PathBuf,

    /// File from which to load words
    #[arg(short, long, default_value = "assets/wordlist.txt")]
    wordlist_file: PathBuf,

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
    word_max_length: u8
}

fn main() {
    let args = Args::parse();

    println!("Hello {}!", args.output_file.display());

    let wordlist = get_word_list(args.wordlist_file);
    // println!("{:#?}", wordlist);
    
    let mut passwords = Vec::new();
    for _ in 0..args.number_passwords {
        passwords.push(generate_password(&wordlist, args.password_word_count, &args.separator));
    }

    println!("{:?}", passwords);
    println!("Generated {} passwords, saved to the file \"{}\".", args.number_passwords, &args.output_file.to_string_lossy());
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
        words.push(wordlist[
            rng.gen_range(0..wordlist_length)].clone());
    }

    words.join(separator)
}

