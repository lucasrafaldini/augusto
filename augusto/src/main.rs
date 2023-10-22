use std::collections::HashSet;
use std::env;
mod wordsmith;


fn main() {
    // let mode = env::args().nth(1).expect("Missing mode");
    let input = env::args().nth(1).expect("Missing input");
    // let input = env::args().nth(2).expect("Missing input");

    // let modes = {
    //     "anagram" : wordsmith::anagram(&input);
    //     "permutation" : wordsmith::permutation(&input);
    // };

    // let result = &modes.get(mode).expect("Invalid mode");
    let result = wordsmith::permutation(&input);

    let set: HashSet<_> = result.into_iter().collect();
    let set_len = set.len();

    println!("{:?}", &set);
    println!("Your word has {:?} combinations", &set_len);
    println!("This is equivalent to {:?}!", &input.len());
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_permutation_combinations() {
        let input = "aba";
        let result = wordsmith::permutation(input);

        // Define the expected set of combinations as owned String values
        let expected: HashSet<String> = vec![
            "aab".to_string(), "baa".to_string(), "aba".to_string(),
        ]
        .into_iter()
        .collect();

        // Convert the result into a HashSet for comparison
        let result_set: HashSet<String> = result.into_iter().collect();
        println!("{:?}", result_set);

        assert_eq!(result_set, expected);
    }

    #[test]
    fn test_print_words_graphic() {
        let words = vec!["augusto".to_string(), "augusto".to_string(), "augusto".to_string()];
        wordsmith::print_words_graphic(&words);
    }

}
