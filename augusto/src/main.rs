use std::collections::HashSet;
mod anagram;


fn main() {
    let result = anagram::letter_combinations("anagram");

    let set: HashSet<_> = result.into_iter().collect();
    println!("{:?}", set);
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_anagram_combinations() {
        let input = "aba";
        let result = anagram::letter_combinations(input);

        // Define the expected set of combinations as owned String values
        let expected: HashSet<String> = vec![
            "aab".to_string(), "baa".to_string(), "aba".to_string(),
            // ... (add more expected combinations as needed)
        ]
        .into_iter()
        .collect();

        // Convert the result into a HashSet for comparison
        let result_set: HashSet<String> = result.into_iter().collect();

        assert_eq!(result_set, expected);
    }
}
