use finding_nemo::{find_word_in_sentence, print_found_keyword};
use std::io;

const QUIT: &str = "QuitApp";

fn main() {
    println!("To quit enter {} as the keyword.", QUIT);

    loop {
        // Messages to use to enter key:
        let mut keyword = String::new();
        println!("Enter keyword");
        io::stdin()
            .read_line(&mut keyword)
            .expect("Error input, could not read line");
        let keyword = keyword.trim(); // Trim the keyword to remove newline characters

        // Break out of loop and quit if required
        match find_word_in_sentence(QUIT, &keyword) {
            None => {}
            Some(_) => break,
        }

        // Messages to use to enter sentence to search
        let mut test_case = String::new();
        println!("Enter Test sentence: ");
        io::stdin()
            .read_line(&mut test_case)
            .expect("Error input, could not read line");

        // Call function in lib.rs and print result
        println!("{}", print_found_keyword(keyword, &test_case));
    }
}
