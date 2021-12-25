use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::fs;
use std::io::{BufRead, BufReader};
use tch;
use std::fmt;

mod tokenizer;
mod inflect_number;

use crate::fast_speech::tokenizer::{Tokenizer, TwitterTokenizer};


lazy_static! {
    static ref TAB: Regex = Regex::new(r"\t").unwrap();
    static ref WHITESPACES: Regex = Regex::new(r"\s+").unwrap();
    static ref NOT_SUPPORTED: Regex = Regex::new(r"[^ a-z'.,?!-]").unwrap();
    static ref NOT_PROCESS: Regex = Regex::new(r"[^a-z]").unwrap();
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

#[derive(Clone, PartialEq)]
pub struct Sound {
    phonemes: Vec<String>,
    graphemes: Option<String>,
}

impl Sound {
    pub fn new(phonemes: Vec<String>) -> Self {
        Self {
            phonemes,
            graphemes: None
        }
    }

    pub fn new_with_graphemes(phonemes: &str, graphemes: &str) -> Self {
        let mut phonemes : Vec<&str> = phonemes.split(' ').collect();
        let phonemes = phonemes.iter_mut().map( |el| {
            el.to_string()
        }).collect();

        Self {
            phonemes,
            graphemes: Some(graphemes.to_string())
        }
    }
}

impl fmt::Display for Sound {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Use `self.number` to refer to each positional data point.
        let graphemes =  match &self.graphemes {
            Some(grap) => grap.as_str(),
            None => ""
        };

        write!(f, "({}, {:?})", graphemes, self.phonemes)
    }
}

pub struct PhonemsProcessor {
    lexicon: HashMap<String, Sound>,
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
            let v: Vec<&str> = TAB.split(line).collect();

            assert_eq!(v.len(), 2);
            
            let graphemes = v[0].trim().to_lowercase();
            let phonemes = v[1].trim();
            let sound = Sound::new_with_graphemes(phonemes, &graphemes);

            lexicon.entry(
                graphemes
            ).or_insert(
                sound
            );
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

    pub fn get(&self, key: &str) -> Option<&Sound> {
        self.lexicon.get(key)
    }

    fn predict(&self, token: &str) -> Result<Vec<String>, PredictionError> {
        Ok(vec!["".to_string()])
    }

    pub fn phonemizer(&self, text: &str) -> Vec<String> {
        // TODO: remove double spaces
        let text = text.replace(|c: char| !c.is_ascii(), "");
        let text = inflect_number::normalize_number(&text).unwrap().to_lowercase();

        let text = NOT_SUPPORTED.replace_all(&text, "");
        
        let text = text.as_ref().replace("i.e.", "that is");
        let text = text.replace("e.g.", "for example");

        let tokens: Vec<&str> = self.tokenizer.tokenize(&text).collect();

        let mut prons: Vec<String> = Vec::new();

        for t in tokens {

            // handle symbols
            if NOT_PROCESS.is_match(t) {
                let pron = vec![t.to_string()];
                prons.extend(pron);
            }
            // handle words
            else {
                let pron = match self.get(t) {
                    Some(sound) => sound.phonemes.clone(),
                    None => self.predict(t).unwrap()
                };

                prons.extend(pron);
            }
        }

        prons
    }
}

#[cfg(test)]
mod test {
    use super::*;


    #[test]
    fn test_lexicon_read() {
        let processor = PhonemsProcessor::build(
            "./resources/lexicon.txt"
        );

        assert_eq!(processor.len(), 200020);
    }

}
