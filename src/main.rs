use clap::Parser;
use rand::Rng;
use std::{
    fs,
    fs::File,
    io::{BufRead, BufReader, Write},
    path::PathBuf, process,
};

#[derive(Parser, Debug)]
#[command(version, about = "Generate passwords comprised of multiple random words.", long_about = None)]
struct Args {
    /// File in which to save the generated passwords
    #[arg(short, long, default_value = "passwords.txt")]
    output_file: PathBuf,

    /// Optional file from which to load words (overriding the internal word list)
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

    /// Dump internal word list to "wordlist.txt", then exit.
    #[arg(short, long, default_value_t = false)]
    dump_internal_words: bool,

    /// Minimum length of words used
    #[arg(long, default_value_t = 3)]
    word_min_length: usize,

    /// Maximum length of words usedline
    #[arg(long, default_value_t = 6)]
    word_max_length: usize,
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();
    let mut wordlist:Vec<String>= Vec::new();

    if args.wordlist_file.is_none() {
        // Load from asset file included into binary.
        let wordlist_str = include_str!("assets/wordlist.txt");
        if args.dump_internal_words {
            // println!("{}", wordlist.join(","));
            // let mut dump_file = File::create("wordlist.txt")?;

            fs::write("wordlist.txt", wordlist_str)?;
            println!("Dumped internal word list to \"wordlist.txt\".");
            process::exit(0);
        }
        println!("Using internal word list.");
        for line in wordlist_str.lines() {
            if line.char_indices().count() >= args.word_min_length
                && line.char_indices().count() <= args.word_max_length
            {
                wordlist.push(line.to_string());
            }
        }
    } else {
        // Load from arg.
        let listfile = args.wordlist_file.unwrap();
        println!("Using specified word list: {}", listfile.display());
        wordlist = get_word_list(listfile, args.word_min_length, args.word_max_length);
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
fn get_word_list(filename: PathBuf, word_min_length: usize, word_max_length: usize) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|line| line.expect("Could not parse line"))
        .filter(|line| {
            line.char_indices().count() >= word_min_length
                && line.char_indices().count() <= word_max_length
        })
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
