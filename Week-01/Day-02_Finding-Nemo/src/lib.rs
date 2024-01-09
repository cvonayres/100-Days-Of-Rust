pub fn print_found_keyword(key: &str, value: &str) -> String {
    match find_word_in_sentence(key, value) {
        Some((index, word)) => format!("I found {} at {}!", word, index),
        None => format!("I can't find {} :(", key),
    }
}

pub fn find_word_in_sentence(word: &str, sentence: &str) -> Option<(usize, String)> {
    sentence
        .split_whitespace()
        .enumerate()
        .find(|&(_, val)| val == word)
        .map(|(i, val)| (i + 1, val.to_string())) // Add 1 to i to start counting from 1
}

#[cfg(test)]
mod test {
    use super::*;

    // findNemo("I am finding Nemo !") ➞ "I found Nemo at 4!"
    #[test]
    fn test_examples_1() {
        let key = "Nemo";
        let test_sentence = "I am finding Nemo !";
        let correct_result = "I found Nemo at 4!";

        assert_eq!(print_found_keyword(key, test_sentence), correct_result);
    }

    // findNemo("Nemo is me") ➞ "I found Nemo at 1!"
    #[test]
    fn test_examples_2() {
        let key = "Nemo";
        let test_sentence = "Nemo is me";
        let correct_result = "I found Nemo at 1!";

        assert_eq!(print_found_keyword(key, test_sentence), correct_result);
    }

    // findNemo("I Nemo am") ➞ "I found Nemo at 2!"
    #[test]
    fn test_examples_3() {
        let key = "Nemo";
        let test_sentence = "I Nemo am";
        let correct_result = "I found Nemo at 2!";

        assert_eq!(print_found_keyword(key, test_sentence), correct_result);
    }

    // If you can't find Nemo, return I can't find Nemo :(
    #[test]
    fn test_examples_4() {
        let key = "Nemo";
        let test_sentence = "I am finding Noname !";
        let correct_result = "I can't find Nemo :(";

        assert_eq!(print_found_keyword(key, test_sentence), correct_result);
    }

    // Nemo will always look like Nemo, and not NeMo or other capital variations.
    #[test]
    fn test_examples_5() {
        let key = "Nemo";
        let test_sentence = "I am finding NeMo !";
        let correct_result = "I found Nemo at 4!";

        assert_ne!(print_found_keyword(key, test_sentence), correct_result);
    }

    // Nemo's, or anything that says Nemo with something behind it, doesn't count as Finding Nemo.
    #[test]
    fn test_examples_6() {
        let key = "Nemo";
        let test_sentence = "I am finding Nemo's !";
        let correct_result = "I found Nemo at 4!";
        assert_ne!(print_found_keyword(key, test_sentence), correct_result);
    }

    // If there are multiple Nemo's in the sentence, only return for the first one.
    #[test]
    fn test_examples_7() {
        let key = "Nemo";
        let test_sentence = "I am finding Nemo & Nemo its a wild part !";
        let correct_result = "I found Nemo at 4!";

        assert_eq!(print_found_keyword(key, test_sentence), correct_result);
    }
}
