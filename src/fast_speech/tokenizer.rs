use regex::Regex;
use std::fmt;

pub trait Tokenizer: fmt::Debug {
    // taken from https://github.com/rth/vtext/blob/main/src/tokenize/mod.rs
    fn tokenize<'a>(&'a self, text: &'a str) -> Box<dyn Iterator<Item = &'a str> + 'a>;
}

#[derive(Clone, Debug)]
pub struct RegexpTokenizer {
    pub pattern: String,
    regexp: Regex,
}

impl RegexpTokenizer {
    fn build(pattern: &str) -> Self {
        Self {
            pattern: pattern.to_string(),
            regexp: Regex::new(pattern).unwrap(),
        }
    }
}

impl Default for RegexpTokenizer {
    /// Create a new instance
    fn default() -> Self {
        let pattern = String::from(r"\b\w\w+\b");
        RegexpTokenizer::build(pattern.as_str())
    }
}

impl Tokenizer for RegexpTokenizer {
    /// Tokenize a string
    fn tokenize<'a>(&'a self, text: &'a str) -> Box<dyn Iterator<Item = &'a str> + 'a> {
        Box::new(self.regexp.find_iter(text).map(|m| m.as_str()))
    }
}

/// Regular expression tokenizer
#[derive(Clone, Debug)]
pub struct TwitterTokenizer {
    pub patterns: Vec<String>,
    regexp: Regex,
}

impl Default for TwitterTokenizer {
    /// Create a new instance
    fn default() -> Self {
        let patterns = vec![
            String::from(
                r#"(?:[<>]?[:;=8][\-o\*']?[\)\]\(\[dDpP/:\}\{@\|\\]|[\)\]\(\[dDpP/:\}\{@\|\\][\-o\*']?[:;=8][<>]?|</?3)"#,
            ),
            String::from(
                r#"(?x)
                (?:
                    https?:
                    (?:
                        /{1,3}
                        |
                        [a-z0-9%]
                    )
                    |
                    [a-z0-9.\-]+[.]
                    (?:[a-z]{2,13})
                /
                )
                (?:
                    [^\s()<>{}\[\]]+
                    |
                    \([^\s()]*?\([^\s()]+\)[^\s()]*?\)
                    |
                    \([^\s]+?\)
                )+
                (?:
                    \([^\s()]*?\([^\s()]+\)[^\s()]*?\)
                    |
                    \([^\s]+?\)
                    |
                    [^\s`!()\[\]{};:'".,<>?«»“”‘’]
                )
                "#,
            ),
            // HTML tags
            String::from(r#"<[^>\s]+>"#),
            // ASCII Arrows
            String::from(r#"[\-]+>|<[\-]+"#),
            // Twitter username:
            String::from(r#"(?:@[\w_]+)"#),
            // Twitter hashtags:
            String::from(r#"(?:\#+[\w_]+[\w'_\-]*[\w_]+)"#),
            // email addresses
            String::from(r#"[\w.+-]+@[\w-]+\.(?:[\w-]\.?)+[\w-]"#),
            // Zero-Width-Joiner and Skin tone modifier emojis
            String::from(
                r#".(?:[\U0001F3FB-\U0001F3FF]?(?:\u200d.[\U0001F3FB-\U0001F3FF]?)+|[\U0001F3FB-\U0001F3FF])"#,
            ),
            // Remaining word types
            String::from(
                r#"(?:[^\W\d_](?:[^\W\d_]|['-_])+[^\W\d_])|(?:[+\-]?\d+[,/.:-]\d+[+\-]?)|(?:[\w_]+)|(?:\.(?:\s*\.){1,})|(?:\S)"#,
            ),
        ];

        let regexp = Regex::new(patterns.join("|").as_str()).unwrap();

        Self { patterns, regexp }
    }
}

impl Tokenizer for TwitterTokenizer {
    /// Tokenize a string
    fn tokenize<'a>(&'a self, text: &'a str) -> Box<dyn Iterator<Item = &'a str> + 'a> {
        Box::new(self.regexp.find_iter(text).map(|m| m.as_str()))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_tweet_tokenizer() {

        let tokenizer = TwitterTokenizer::default();
        let tokens: Vec<&str> = tokenizer
            .tokenize("This is a cooool #dummysmiley: :-) :-P <3 and some arrows < > -> <--")
            .collect();
        let expected: &[_] = &[
            "This",
            "is",
            "a",
            "cooool",
            "#dummysmiley",
            ":",
            ":-)",
            ":-P",
            "<3",
            "and",
            "some",
            "arrows",
            "<",
            ">",
            "->",
            "<--",
        ];
        assert_eq!(tokens, expected);
        println!("{:?}", tokens);
    }

    #[test]
    fn test_tweet_tokenizer_usernname() {
        let tokenizer = TwitterTokenizer::default();
        let tokens: Vec<&str> = tokenizer
            .tokenize("@remy: This is way too much for you!!!")
            .collect();
        let expected: &[_] = &[
            "@remy", ":", "This", "is", "way", "too", "much", "for", "you", "!", "!", "!",
        ];
        println!("{:?}", tokens);
        assert_eq!(tokens, expected);
    }
}
