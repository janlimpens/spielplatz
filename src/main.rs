#![feature(hash_set_entry)]
#![feature(entry_insert)]
#[allow(dead_code)]
use regex::Regex;
use std::{
    collections::{HashMap, HashSet}
};
mod r#async;
fn main() {
    Bayes::new();
}

struct Bayes {
    matrix: HashMap<Categorization, u32>,
    ignore_words: HashSet<String>,
    tokenizer: Regex,
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
            tokenizer: Regex::new(r#"[\w'-]+"#).unwrap()
        };
        bayes.load_ignore_words();
        bayes
    }

    fn load_ignore_words(&mut self) {
        for word in "be i am are is was were a an this the and or so much not no nor do don't"
            .to_string()
            .split(" ")
        {
            self.ignore_words.insert(word.to_string());
        }
    }

    /// chops a string into word like pieces
    pub fn tokenize(&self, text: &str) -> Vec<String> {
        let text = text.to_lowercase();
        self.tokenizer
            .find_iter(&text)
            .map(|m| m.as_str().to_string())
            .collect::<Vec<String>>()
    }

    /// manually categorize a text
    pub fn learn(&mut self, text: &str, bucket: &str) {
        if text == String::default() || bucket == String::default() {
            return;
        }
        let bucket = &bucket.to_lowercase();
        for word in self.tokenize(text) {
            self.add_word_to_bucket(&word, bucket);
        }
    }

    pub fn add_word_to_bucket(&mut self, word: &str, bucket: &str) {
        if word == String::default() || self.ignore_words.contains(word) {
            return;
        }
        let key = Categorization { word: word.to_string(), bucket: bucket.to_string() };
        let entry = self.matrix.entry(key).or_insert(0);
        *entry += 1;
    }

    fn get_scores(&self, word: &str) -> Vec<(String, u32)> {
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
        x.sort_by(|(_, a), (_, b)| b.cmp(&a));
        x
    }

    fn display_matrix(&self) {
        for (word, score) in &self.matrix {
            println!("Key: {}/{} Val:{}", word.word, word.bucket, score);
        }
    }

    fn guess_bucket(&self, text: &str) -> Vec<String> {
        if text == String::default() {
            return vec![];
        }
        let mut bucket_score: HashMap<String, u32> = HashMap::new();
        for token in self.tokenize(text) {
            let scores = self.get_scores(&token);
            for (bucket, score) in scores {
                let entry = bucket_score.entry(bucket).or_default();
                *entry += score;
            }
        }
        let mut result: Vec<(&String, &u32)> = bucket_score.iter().collect();
        result.sort_by(|(_, score_a), (_, score_b)| score_b.cmp(&score_a));
        if result.is_empty() {
            return Vec::new();
        }
        let (_, &highscore) = result[0];
        return result
            .iter()
            .filter(|(_, &score)| score == highscore)
            .map(|(bucket, _)| bucket.to_string())
            .collect::<Vec<String>>();
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
        learning_body.iter().for_each(|(text, bucket)| {
            bayes.learn(text, bucket);
        });
        bayes
    }

    #[test]
    fn test_tokenizer() {
        let bayes = Bayes::new();
        let matches = bayes.tokenize("some words, some with áccents
        (and hyphen-separated) 2021  !");
        assert_eq!(matches.into_iter().count(), 8);
    }

    #[test]
    fn test_learn() {
        let bayes = get_taught_bayes();
        bayes.display_matrix();
        assert!(bayes.get_scores("i").is_empty());
        let (bucket, score) = &bayes.get_scores("reads")[0];
        assert_eq!(score, &2);
        assert_eq!(bucket, "book");
    }

    #[test]
    fn get_scores_returns_results_in_good_order() {
        let bayes = get_taught_bayes();
        let scores_for_reads = bayes.get_scores("reads");
        for (bucket, score) in &scores_for_reads {
            println!("Bucket: '{bucket}' Score: {score}")
        }
        assert_eq!(scores_for_reads[0].1, 2u32);
        assert_eq!(scores_for_reads[1].1, 1u32);
    }

    #[test]
    fn get_a_good_guess() {
        let bayes = get_taught_bayes();
        let assumed_buckets =
            bayes.guess_bucket("Miss so and so visits the classical library");
        assert_eq!(assumed_buckets[0], "book".to_string());
        let assumed_buckets =
            bayes.guess_bucket("He goes to the store and reads filthy publications");
        assert_eq!(assumed_buckets[0], "magazine".to_string());
    }
}
