use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::fs;
use std::io::{BufRead, BufReader};
use tch;

mod tokenizer;
mod inflect_number;

use crate::fast_speech::tokenizer::{Tokenizer, TwitterTokenizer};


lazy_static! {
    static ref WHITESPACES: Regex = Regex::new(r"\s+").unwrap();
    static ref NON_DIGIT: Regex = Regex::new(r"\s+").unwrap();
    static ref NON_SUPPORTED: Regex = Regex::new(r"[^ a-z'.,?!-]").unwrap();
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

#[derive(Debug, Clone)]
pub enum PredictionError {
    ModelError
}

pub struct PhonemsProcessor {
    lexicon: HashMap<String, String>,
    tokenizer: TwitterTokenizer
}

impl<'a> PhonemsProcessor {
    pub fn build(path: &str) -> Self {
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

        Self { 
            lexicon,
            tokenizer: TwitterTokenizer::default()
        }
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

    fn predict(&self, token: &str) -> Result<String, PredictionError> {
        Ok("".to_string())
    }

    pub fn phonemizer(&self, text: &str) -> Vec<String> {
        let text = text.replace(|c: char| !c.is_ascii(), "");
        let text = inflect_number::normalize_number(&text).unwrap().to_lowercase();

        let text = NON_SUPPORTED.replace_all(&text, "");
        
        let text = text.as_ref().replace("i.e.", "that is");
        let text = text.replace("e.g.", "for example");

        let tokens: Vec<&str> = self.tokenizer.tokenize(&text).collect();

        let mut prons: Vec<String> = Vec::new();

        for t in tokens {

            let pron = match self.get(t) {
                Some(phonem) => phonem.clone(),
                None => self.predict(t).unwrap()
            };

            prons.push(pron);
            prons.push(" ".to_string());
        }

        prons
    }
}

#[cfg(test)]
mod test {
    use super::*;
}
