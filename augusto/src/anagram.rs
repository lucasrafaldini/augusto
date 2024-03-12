pub fn letter_combinations(input: &str) -> Vec<String> {
    if input.len() <= 1 {
        return vec![input.to_string()];
    }

    let mut result:Vec<String> = Vec::new();

    for (i, char) in input.chars().enumerate() {
        let rest = input.chars().enumerate().filter_map(|(j, c)| {
            if i != j {
                Some(c)
            } else {
                None
            }
        }).collect::<String>();

        for anagram in letter_combinations(&rest) {
            result.push(format!("{}{}", char, anagram));
        }
    }

    result
}
