mod hangman;

use crate::hangman::Hangman;

fn main() {
    let hangman = Hangman::new(&"hello".to_string());

    println!("{:?}", &hangman);
}
