mod errors;
mod types;
use std::{
    error::Error,
    fs::File,
    io::{prelude::*, BufReader},
};
use types::Spellchecker;

const INPUT_FILE: &str = "loc_test.csv";

fn read_file(filepath: &str, index: Option<usize>) -> Result<Vec<String>, Box<dyn Error>> {
    let mut suspicious_words: Vec<String> = Vec::new();
    let mut spell = Spellchecker::new();
    if filepath.contains(".csv") {
        let mut reader = csv::ReaderBuilder::new().from_path(filepath)?;
        for result in reader.records() {
            let record = result?;
            match index {
                Some(s) => {
                    if let Some(r) = record.get(s) {
                        let result = spell.read(r);
                        match result {
                            Ok(res) => {
                                suspicious_words.extend(res);
                            }
                            Err(e) => {
                                // println!("{}", e);
                            }
                        }
                    }
                }
                None => {
                    for idx in 0..record.len() {
                        if let Some(r) = record.get(idx) {
                            let result = spell.read(r);
                            match result {
                                Ok(res) => {
                                    suspicious_words.extend(res);
                                }
                                Err(e) => {
                                    // println!("{}", e);
                                }
                            }
                        }
                    }
                }
            }
        }
    } else if filepath.contains(".txt") {
        let file = File::open(filepath)?;
        let reader = BufReader::new(file);

        for line in reader.lines().flatten() {
            let result = spell.read(&line);
            match result {
                Ok(res) => {
                    suspicious_words.extend(res);
                }
                Err(e) => {
                    // println!("{}", e);
                }
            }
        }
    }
    spell.write_database_to_file();
    Ok(suspicious_words)
}

fn main() {
    let result = read_file(INPUT_FILE, Some(2));
    match result {
        Ok(res) => {
            println!("\nWords with possible typos:\n");
            for w in res {
                println!("{}", w);
            }
            println!("\nDone.");
        }
        Err(e) => {
            println!("{}", e);
        }
    }
}
