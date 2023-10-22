pub fn permutation(input: &str) -> Vec<String> {
    if input.len() <= 1 {
        return vec![input.to_string()];
    }

    let mut result = Vec::new();

    for (i, char) in input.chars().enumerate() {
        let rest = input.chars().enumerate().filter_map(|(j, c)| {
            if i != j {
                Some(c)
            } else {
                None
            }
        }).collect::<String>();

        for permutation in permutation(&rest) {
            result.push(format!("{}{}", permutation, char));
        }
    }

    result
}

use std::cmp;

pub fn print_words_graphic(words: &[String]) {
    let max_word_len = words.iter().map(|w| w.len()).max().unwrap_or(0);
    let max_count = words.iter().map(|w| w.chars().count()).max().unwrap_or(0);

    for i in 0..max_count {
        for word in words {
            let padding = " ".repeat(cmp::max(max_word_len - word.len(), 0));
            let ch = word.chars().nth(i).unwrap_or(' ');
            print!("{}{} ", ch, padding);
        }
        println!();
    }
}
