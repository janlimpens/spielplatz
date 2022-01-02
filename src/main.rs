#![feature(hash_set_entry)]
#![feature(entry_insert)]
#[allow(dead_code)]
use regex::Regex;
use std::{collections::{HashMap, HashSet}, ops::ControlFlow};
mod r#async;
fn main() {
    Bayes::new();
}

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
        for word in "be i am are is was were a an this the and or so much not no nor"
            .to_string()
            .split(" ")
        {
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
            let word = word.as_str().to_string();
            let bucket = bucket.clone();
            self.add_word_to_bucket(word, bucket);
        }
    }

    fn add_word_to_bucket(&mut self, word: String, bucket: String) {
        if word == String::default() || self.ignore_words.contains(&word) {
            return;
        }
        let key = Categorization {
            word,
            bucket,
        };
        let entry = self.matrix.entry(key).or_insert(0);
        *entry += 1;
    }

    fn get_scores(&self, word: String) -> Vec<(String, u32)> {
        let mut x: Vec<(String, u32)> = self
            .matrix
            .keys()
            .filter(|&cat| cat.word == word)
            .map(|cat| {
                let score = self.matrix.get(&cat).unwrap().to_owned();
                let bucket = cat.bucket.clone();
                (bucket, score)
            })
            .collect();
        x.sort_by(|a, b| b.1.cmp(&a.1));
        x
    }

    fn display(&self) {
        for (word, score) in &self.matrix {
            println!("Key: {}/{} Val:{}", word.word, word.bucket, score);
        }
    }

    fn guess_bucket(&self, text: String) -> String {
        if text == String::default() {
            return text;
        }
        let regex = Regex::new("([a-z]*)").unwrap();
        let matches = regex.find_iter(&text);
        let mut bucket_score: HashMap<String, u32> = HashMap::new();
        for token in matches {
            let scores = self.get_scores(token.as_str().to_string());
            for (bucket, score) in scores {
                let entry = bucket_score.entry(bucket).or_default();
                *entry += score;
            }
        }
        let mut result: Vec<(&String, &u32)> = bucket_score.iter().collect();
        result.sort_by(|a, b| b.cmp(&a));
        if result.is_empty() {
            return String::default();
        }
        for r in &result {
            println!("{} with score {}", r.0, r.1);
        }
        result[0].0.to_owned()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    fn get_taught_bayes() -> Bayes {
        let learning_body = [
            ("The child wants to read a book", "book"),
            (
                "Reading books and magazines make for a great pass-time",
                "book",
            ),
            ("Miss Darlington reads a book in the afternoon", "book"),
            ("There is whole library full of books", "book"),
            ("All you do is reading those filthy magazines!", "magazine"),
            ("The child reads a classical novel", "book"),
            ("He reads a generonormative magazine", "magazine"),
            ("Magazines are great!", "magazine"),
            ("The book was clad book in exquisit leather", "book"),
            ("In the filthy store they sell magazines", "magazine"),
        ];
        let mut bayes = Bayes::new();
        learning_body.iter().for_each(|item| {
            bayes.learn(item.0.to_string(), item.1.to_string());
        });
        bayes
    }

    #[test]
    fn test_learn() {
        let bayes = get_taught_bayes();
        bayes.display();
        assert!(bayes.get_scores("i".to_string()).is_empty());
        assert_eq!(bayes.get_scores("reads".to_string()).first().unwrap().1, 2);
    }

    #[test]
    fn get_scores_returns_results_in_good_order() {
        let bayes = get_taught_bayes();
        let book_scores = bayes.get_scores("reads".to_string());
        for score in &book_scores {
            println!("Bucket: '{}' Score: {}", score.0, score.1)
        }
        assert_eq!(book_scores.first().unwrap().1.to_owned(), 2);
        assert_eq!(book_scores[1].1.to_owned(), 1);
    }

    #[test]
    fn get_a_good_guess() {
        let bayes = get_taught_bayes();
        let assumed_bucket =
            bayes.guess_bucket("Miss so and so visits the classical library".to_string());
        assert_eq!(assumed_bucket, "book".to_string());
        let assumed_bucket =
            bayes.guess_bucket("He goes to the store and reads filthy publications".to_string());
        assert_eq!(assumed_bucket, "magazine".to_string());
    }
}
