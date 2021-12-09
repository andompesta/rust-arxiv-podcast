use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::fs;
use std::io::{BufRead, BufReader};
use tch;

lazy_static! {
    static ref WHITESPACES: Regex = Regex::new(r"\s+").unwrap();
    static ref NON_DIGIT: Regex = Regex::new(r"\s+").unwrap();
}

pub fn load_model(path: &str) -> tch::CModule {
    let model = match tch::CModule::load(path) {
        Ok(model) => model,
        Err(e) => {
            panic!("{}", e);
        }
    };

    model
}

pub struct PhonemsProcessor {
    lexicon: HashMap<String, String>,
}

impl PhonemsProcessor {
    pub fn build(path: &str) -> PhonemsProcessor {
        let file = fs::File::open(path).unwrap();
        let reader = BufReader::new(file);
        let mut lexicon = HashMap::new();

        for line in reader.lines() {
            let line = line.unwrap();
            let line = line.trim();
            let v: Vec<&str> = WHITESPACES.splitn(line, 2).collect();

            let word = v[0].trim().to_lowercase().to_string();
            let phonem = v[1].trim().to_string();

            lexicon.entry(word).or_insert(phonem);
        }

        PhonemsProcessor { lexicon }
    }

    pub fn len(&self) -> usize {
        self.lexicon.len()
    }

    pub fn is_empty(&self) -> bool {
        self.lexicon.is_empty()
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        self.lexicon.get(key)
    }

    pub fn num_to_word(&self, num: &str) -> String {
        let rev_num = num.chars().rev().collect::<String>();
        let sign = num.as_bytes()[0];

        let sign = match sign {
            b'+' => "plus",
            b'-' => "minus",
            _ => "",
        };

        let my_ord = matches!(&rev_num[0..2], "st" | "nd" | "rd" | "th");

        let mut chunks: Vec<&str> = num.splitn(2, '.').collect();

        let mut first = 1;
        let mut loopstart = 0;
        let first_element = chunks.first().unwrap();

        if first_element.is_empty() {
            first = 0;
            if chunks.len() > 1 {
                loopstart = 1;
            }
        }

        for (i, chunk) in chunks.iter().enumerate() {}

        return "".to_string();
    }
}
