use super::errors::Error;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

const DATABASE_FILE: &str = "wordlist.csv";

#[derive(Debug, Serialize, Deserialize)]
struct Entry {
    word: String,
    instances: Option<u32>,
}

pub struct Spellchecker {
    database: HashMap<String, u32>,
    regex: Regex,
}

impl Spellchecker {
    pub fn new() -> Self {
        let reg = Regex::new(r"[\\bA-Za-z\\'’-]+").unwrap();
        let data = read_database_from_file();
        Self {
            database: data,
            regex: reg,
        }
    }

    pub fn read(&mut self, text: &str) -> Result<Vec<String>, Error> {
        let mut possible_typos: Vec<String> = Vec::new();
        for word in self.regex.find_iter(&text.to_lowercase().replace('’', "'")) {
            if word.as_str().chars().all(char::is_numeric)
                || !word.as_str().chars().all(char::is_alphabetic)
                || word.as_str().is_empty()
            {
                continue;
            }

            let count = self.database.entry(word.as_str().to_string()).or_insert(0);
            *count += 1;

            if let Some(val) = self.database.get(word.as_str()) {
                let value = usize::try_from(val.to_owned()).unwrap();
                if value < ((self.database.len() / 100) as f32 * 0.05) as usize {
                    possible_typos.push(word.as_str().to_string());
                }
            }
        }

        if possible_typos.is_empty() {
            Err(Error::NoRegexMatches)
        } else {
            Ok(possible_typos)
        }
    }

    pub fn add_word_to_database(&mut self, word: String) {
        let count = self.database.entry(word).or_insert(0);
        *count += 1;
    }

    pub fn print_database(&self) {
        for i in self.database.iter() {
            println!("{} : {}", i.0, i.1);
        }
    }

    pub fn print_database_with_threshold(&self, threshold: u32) {
        for i in self.database.iter() {
            if i.1 <= &threshold {
                println!("{} : {}", i.0, i.1);
            }
        }
    }

    pub fn write_database_to_file(&self) {
        let mut wtr = csv::Writer::from_path(DATABASE_FILE).unwrap();
        for i in self.database.iter() {
            wtr.serialize(Entry {
                word: i.0.to_string(),
                instances: Some(*i.1),
            })
            .unwrap();
        }
        wtr.flush().unwrap();
    }
}

fn create_new_database_file() {
    let mut wtr = csv::Writer::from_path(DATABASE_FILE).unwrap();
    wtr.serialize(Entry {
        word: String::new(),
        instances: Some(0),
    })
    .unwrap();
    wtr.flush().unwrap();
}

fn read_database_from_file() -> HashMap<String, u32> {
    let reader = csv::Reader::from_path(DATABASE_FILE);
    match reader {
        Err(_) => {
            create_new_database_file();
            read_database_from_file()
        }
        Ok(mut rea) => {
            let mut database: HashMap<String, u32> = HashMap::new();
            for result in rea.deserialize() {
                let record: Entry = result.unwrap();
                database.insert(record.word, record.instances.unwrap());
            }
            database
        }
    }
}
