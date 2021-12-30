#![feature(hash_set_entry)]
use regex::Regex;
use std::collections::{HashMap, HashSet};
fn main() {}

struct Bayes {
    matrix: HashMap<Categorization, u32>,
    ignore_words: HashSet<String>,
}

#[derive(PartialEq, Hash, Eq, Debug, Clone)]
struct Categorization {
    bucket: String,
    word: String,
}

impl Bayes {
    fn new() -> Self {
        let mut bayes = Bayes {
            matrix: HashMap::new(),
            ignore_words: HashSet::new(),
        };
        bayes.load_ignore_words();
        bayes
    }

    fn load_ignore_words(&mut self) {
        for word in "i am are is a an this the".to_string().split(" ") {
            self.ignore_words.insert(word.to_string());
        }
    }

    fn learn(&mut self, text: String, bucket: String) {
        if text == String::default() || bucket == String::default() {
            return;
        }
        let bucket = bucket.to_lowercase();
        let text = text.to_lowercase();
        let regex = Regex::new("([a-z]*)").unwrap();
        let matches = regex.find_iter(&text);
        for word in matches {
            let value = word.as_str().to_string();
            let bucket = bucket.clone();
            if value == String::default() || self.ignore_words.contains(&value) {
                continue;
            }
            let key = Categorization {
                word: value,
                bucket,
            };
            let entry = self.matrix.entry(key).or_insert(0);
            *entry += 1;
        }
    }

    fn get_score(&self, word: String) -> Vec<(String, u32)> {
        self.matrix
            .keys()
            .filter(|&cat| cat.word == word)
            .map(|cat| {
                let score = self.matrix.get(&cat).unwrap().to_owned();
                let bucket = cat.bucket.clone();
                (bucket, score)
            })
            .collect()
    }

    fn display(&self) {
        for (word, score) in &self.matrix {
            println!("Key: {}/{} Val:{}", word.word, word.bucket, score);
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]

    fn test_learn() {
        let learning_body = [
            ("I am a book", "book"),
            ("This is a book", "book"),
            ("She reads a book", "book"),
            ("He reads a generonormative magazine", "magazine"),
            ("Magazines are great!", "magazine"),
            ("I am another book", "book"),
            ("I am a magazine", "magazine"),
        ];

        let mut bayes = Bayes::new();

        learning_body.iter().for_each(|item| {
            bayes.learn(item.0.to_string(), item.1.to_string());
        });

        bayes.display();

        let entry_for_i = bayes.get_score("i".to_string());
        assert!(entry_for_i.is_empty());

        assert_eq!(bayes.get_score("reads".to_string()).first().unwrap().1, 1);
    }
}
