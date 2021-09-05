use tch;
use std::collections::HashMap;
use std::fs;
use std::io::{BufRead, BufReader};
use regex::Regex;

pub fn load_model(path: &str) -> tch::CModule {
    let model = match tch::CModule::load(path) {
        Ok(model) => model,
        Err(e) => {
            panic!("{}", e);
        }
    };

    return model;
}


pub struct PhonemsProcessor {
    lexicon: HashMap<String, String>
}

impl PhonemsProcessor {
    pub fn build(path: &str) -> PhonemsProcessor {
        let file = fs::File::open(path).unwrap();
        let reader = BufReader::new(file);
        let mut lexicon = HashMap::new();
        let seperator = Regex::new(r"\s+").unwrap();

        for (index, line) in reader.lines().enumerate() {
            let line = line.unwrap();
            let line = line.trim();
            let v: Vec<&str> = seperator.splitn(line, 2).collect();
            
            let word = v[0].trim().to_lowercase().to_string();
            let phonem = v[1].trim().to_string();

            if ! lexicon.contains_key(&word) {
                lexicon.insert(word, phonem);
            }
        }

        return PhonemsProcessor{lexicon:lexicon};
    }

    pub fn len(&self) -> usize {
        return self.lexicon.len();
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        return self.lexicon.get(key);
    }
}