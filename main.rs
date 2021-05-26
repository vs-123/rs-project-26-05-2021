/*
* PROJECT: rs-project-26-05-2021
* DESCRIPTION: Given letters with their values as follows a = 1, b = 2, c = 3 till z = 26, add each of the letters according to their value and return it.
* AUTHOR: Vahin Sharma
*/

use std::collections::HashMap;

fn solution(word: &str) -> u32 {
    word.chars().map(|c| c as u32 - 96).sum()
}

fn main() {
    let question_answers: HashMap<&str, u32> = [
        ("evolution", 133),
        ("friendship", 108),
        ("hippopotomonstrosesquippedaliophobia", 463),
        ("pneumonoultramicroscopicsilicovolcanoconiosis", 560),
    ]
    .iter().cloned().collect();

    let mut marks: u8 = 0;
    for (question, actual_answer) in &question_answers {
        println!("Input: {}", question);
        let answer = solution(question);
        if answer == *actual_answer {
            marks += 1;
            println!("Passed");
        } else {
            println!("Expected {:}, instead got {:}", actual_answer, answer);
        }
        println!();
    }

    println!("Score: {} out of {}", marks, question_answers.keys().len())
}
