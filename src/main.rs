mod errors;
mod types;
use std::{
    error::Error,
    fs::File,
    io::{prelude::*, BufReader},
};
use types::SpellChecker;

const INPUT_FILE: &str = "loc_test.csv";
const TRAINING_FILE: &str = "words.txt";

fn train_with_file(filepath: &str, index: Option<usize>) -> Result<(), Box<dyn Error>> {
    let mut spell = SpellChecker::new();
    if filepath.contains(".csv") {
        let mut reader = csv::ReaderBuilder::new()
            .has_headers(false)
            .from_path(filepath)?;
        for result in reader.records() {
            let record = result?;
            match index {
                Some(s) => {
                    if let Some(r) = record.get(s) {
                        spell.train(r);
                    }
                }
                None => {
                    for idx in 0..record.len() {
                        if let Some(r) = record.get(idx) {
                            spell.train(r);
                        }
                    }
                }
            }
        }
    } else if filepath.contains(".txt") {
        let file = File::open(filepath)?;
        let reader = BufReader::new(file);

        for line in reader.lines().flatten() {
            spell.train(&line);
        }
    }
    // spell.print_database_with_threshold(1);
    spell.increase_training_count();
    spell.write_database_to_file();
    Ok(())
}

fn check_file(filepath: &str, index: Option<usize>) -> Result<Vec<String>, Box<dyn Error>> {
    let mut not_found_words: Vec<String> = Vec::new();
    let mut spell = SpellChecker::new();
    let mut reader = csv::ReaderBuilder::new().from_path(filepath)?;
    for result in reader.records() {
        let record = result?;
        match index {
            Some(s) => {
                if let Some(r) = record.get(s) {
                    let not_found = spell.check(r);
                    match not_found {
                        Ok(res) => {
                            not_found_words.extend(res);
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
                        let not_found = spell.check(r);
                        match not_found {
                            Ok(res) => {
                                not_found_words.extend(res);
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
    spell.gather_learnings();
    spell.increase_training_count();
    spell.write_database_to_file();
    Ok(not_found_words)
}

fn main() {
    // let result = train_with_file(TRAINING_FILE, None);
    // match result {
    //     Ok(res) => {
    //         println!("Done.");
    //     }
    //     Err(e) => {
    //         println!("{}", e);
    //     }
    // }
    let result = check_file(INPUT_FILE, Some(2));
    match result {
        Ok(res) => {
            println!("\nWords with possible typos:\n");
            for w in res {
                println!("{}", w);
            }
        }
        Err(e) => {
            println!("{}", e);
        }
    }
}
