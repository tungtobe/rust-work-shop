use std::fs;
use std::io;

fn main() {
    let mut counter: i32 = 0;
    // Read sample file
    let contents = fs::read_to_string("sample.txt").expect("Something went wrong reading the file");
    println!("File len: {}", contents.len());

    // Read input word
    println!("Please input your word:");
    let mut input_word = String::new();
    io::stdin()
        .read_line(&mut input_word)
        .expect("Failed to read word!");

    // Count
    for word in contents.split_whitespace() {
        if input_word.trim().to_lowercase() == word.to_string().to_lowercase() {
            counter += 1;
        }
    }
    println!("Your word found {} times", counter);
}
