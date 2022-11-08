use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

const DATABASE_FILE: &str = "wordlist.csv";

#[derive(Debug, Serialize, Deserialize)]
struct Entry {
    word: String,
    instances: Option<u32>,
    training_count: Option<u32>,
}

pub struct SpellChecker {
    database: HashMap<String, u32>,
    training_count: u32,
    regex: Regex,
}

impl SpellChecker {
    pub fn new() -> Self {
        let reg = Regex::new(r"[A-Z]{1}[a-z]+|[\\bA-Za-z\\b]+").unwrap();
        let (data, t_count) = read_database_from_file();
        Self {
            database: data,
            training_count: t_count,
            regex: reg,
        }
    }

    pub fn train(&mut self, text: &str) {
        for word in self.regex.find_iter(&text.to_lowercase()) {
            let count = self.database.entry(word.as_str().to_string()).or_insert(0);
            *count += 1;
        }
    }

    pub fn check(&mut self, text: &str) -> bool {
        println!("{}", text);
        let mut found = false;
        for word in self.regex.find_iter(&text.to_lowercase()) {
            println!("- {}", word.as_str());
            if word.as_str().chars().all(char::is_numeric) || word.as_str().is_empty() {
                continue;
            }
            if self.database.contains_key(word.as_str()) {
                found = true;
            }
        }
        found
    }

    pub fn increase_training_count(&mut self) {
        self.training_count += 1;
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
                training_count: Some(self.training_count),
            })
            .unwrap();
        }
        wtr.flush().unwrap();
    }
}

fn read_database_from_file() -> (HashMap<String, u32>, u32) {
    let mut reader = csv::Reader::from_path(DATABASE_FILE).unwrap();
    let mut database: HashMap<String, u32> = HashMap::new();
    let mut training_count: u32 = 0;
    for result in reader.deserialize() {
        let record: Entry = result.unwrap();
        database.insert(record.word, record.instances.unwrap());
        training_count = record.training_count.unwrap();
    }
    (database, training_count)
}