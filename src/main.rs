use std::collections::HashMap;

fn main() {

}

struct Matrix {

}

fn create_matrix() -> HashMap<(&'static str, &'static str), u32> {
    let matrix = HashMap::new();
    matrix
}


fn learn<'a>(text: &'a str, bucket: &'a str, matrix: &mut HashMap<(&'a str, &'a str), u32>) {
    let words = text.split(' ');
    for word in words {
        *matrix.entry((word, bucket)).or_insert(0) += 1;
    }
}

fn display_matrix(matrix: &HashMap<(&str, &str), u32>) {
    for ((text, bucket), val) in matrix {
        println!("Key: {}/{} Val:{}", text, bucket, val);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]

    fn test_learn() {
        let learning_body = [
            ("I am a book", "book"),
            ("I am another book", "book"),
            ("I am a magazine", "magazine")
        ];

        let mut matrix = create_matrix();

        learning_body.iter().for_each(|item| {
            learn(item.0, item.1, &mut matrix);
        });

        display_matrix(&matrix);

        let entry_for_i = matrix.get(&("I", "book")).unwrap();
        assert_eq!(*entry_for_i, 2);
    }
}