pub fn letter_combinations(input: &str) -> Vec<String> {
    if input.len() == 0 {
        return vec!["".to_string()];
    }

    let first_char = &input[0..1];
    let rest = &input[1..];

    let rest_combinations = letter_combinations(rest);

    let mut result = Vec::new();

    for char in first_char.chars() {
        for combination in &rest_combinations {
            result.push(format!("{}{}", char, combination));
        }
    }

    result
}
