use std::collections::HashMap;

fn main() {
    let learning_body = [
        ("I am a book", "book"),
        ("I am another book", "book"),
        ("I am a magazine", "magazine")
    ];

    let mut matrix: HashMap<(&str, &str), i32> = HashMap::new();

    learning_body.iter().for_each(|item| {
        learn(item.0, item.1, &mut matrix);
    });

    display_matrix(matrix);
}

fn learn<'a>(text: &'a str, bucket: &'a str, matrix: &mut HashMap<(&'a str, &'a str), i32>) {
    let words: Vec<&str> = text.split(' ').collect();
    for word in words {
        *matrix.entry((word, bucket)).or_insert(1) += 1;
    }
}

fn display_matrix(matrix: HashMap<(&str, &str), i32>) {
    matrix.keys().for_each(|key| {
        println!("Key: {}/{} Val:{}", key.0, key.1, matrix.get(key).unwrap());
    });
}


