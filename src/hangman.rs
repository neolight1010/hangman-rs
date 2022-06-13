use rand::prelude::IteratorRandom;
use std::collections::HashSet;

use crate::pics::HANGMAN_PICS;

#[derive(Debug, PartialEq)]
enum GameState {
    Playing,
    Win,
    Lose,
}

pub enum GuessResult {
    Correct,
    Incorrect,
}

#[derive(Debug)]
pub struct Hangman {
    word: String,

    game_state: GameState,
    lives: usize,

    letters_to_guess: HashSet<char>,
}

impl Hangman {
    pub fn new(word: &String) -> Self {
        let mut letters_to_guess = HashSet::new();

        for c in word.chars() {
            letters_to_guess.insert(c);
        }

        Self {
            word: word.clone(),

            game_state: GameState::Playing,
            lives: HANGMAN_PICS.len() - 1,

            letters_to_guess,
        }
    }

    pub fn guess_letter(&mut self, letter: char) -> GuessResult {
        match self.letters_to_guess.contains(&letter) {
            true => {
                self.letters_to_guess.remove(&letter);

                if self.letters_to_guess.is_empty() {
                    self.game_state = GameState::Win;
                }

                GuessResult::Correct
            }
            false => {
                self.lives -= 1;

                if self.lives == 0 {
                    self.game_state = GameState::Lose;
                }

                GuessResult::Incorrect
            }
        }
    }

    pub fn get_hint(&self) -> Option<&char> {
        self.letters_to_guess.iter().choose(&mut rand::thread_rng())
    }

    pub fn get_guessed_word(&self) -> String {
        let mut guessed_word = "".to_owned();

        for c in self.word.chars() {
            if self.letters_to_guess.contains(&c) {
                guessed_word.push('_');
                continue;
            }

            guessed_word.push(c);
        }

        guessed_word
    }
}

#[cfg(test)]
mod test {
    use super::{GameState, GuessResult, Hangman};

    #[test]
    fn test_new_hangman_should_have_hashset_with_word_chars() {
        let hangman = Hangman::new(&"aabbbccd".to_string());

        assert!(hangman.letters_to_guess.contains(&'a'));
        assert!(hangman.letters_to_guess.contains(&'b'));
        assert!(hangman.letters_to_guess.contains(&'c'));
        assert!(hangman.letters_to_guess.contains(&'d'));
    }

    #[test]
    fn test_guess_letter_should_return_correct() {
        let mut hangman = Hangman::new(&"test".to_string());

        let result = hangman.guess_letter('t');

        match result {
            GuessResult::Correct => (),
            GuessResult::Incorrect => assert!(false, "Result should be Incorrect."),
        };
    }

    #[test]
    fn test_guess_letter_should_return_incorrect() {
        let mut hangman = Hangman::new(&"test".to_string());

        let result = hangman.guess_letter('z');

        match result {
            GuessResult::Correct => assert!(false, "Result should be Correct."),
            GuessResult::Incorrect => (),
        };
    }

    #[test]
    fn test_correct_guess_letter_removes_from_letters_to_guess() {
        let mut hangman = Hangman::new(&"test".to_string());

        hangman.guess_letter('t');

        assert!(!hangman.letters_to_guess.contains(&'t'));
    }

    #[test]
    fn test_guess_letter_returns_incorrect_when_already_guessed() {
        let mut hangman = Hangman::new(&"tt".to_string());

        hangman.guess_letter('t');
        let result = hangman.guess_letter('t');

        match result {
            GuessResult::Correct => assert!(false, "Result should be Incorrect."),
            GuessResult::Incorrect => (),
        }
    }

    #[test]
    fn test_win_state_when_guessed_all_letters() {
        let mut hangman = Hangman::new(&"te".to_string());

        hangman.guess_letter('t');
        hangman.guess_letter('e');

        assert_eq!(hangman.game_state, GameState::Win);
    }

    #[test]
    fn test_incorrect_guess_decreases_lives() {
        let mut hangman = Hangman::new(&"test".to_string());

        let lives_before = hangman.lives;

        hangman.guess_letter('z');

        let lives_after = hangman.lives;

        assert_eq!(lives_after, lives_before - 1);
    }

    #[test]
    fn test_lose_state_after_last_incorrect_guess() {
        let mut hangman = Hangman::new(&"test".to_string());

        let lives = hangman.lives;

        for _ in 0..lives {
            hangman.guess_letter('z');
        }

        assert_eq!(hangman.game_state, GameState::Lose);
    }

    #[test]
    fn test_hint_returns_letter_from_word() {
        let word = "test".to_string();

        let hangman = Hangman::new(&word);

        for _ in 0..50 {
            let hint = hangman.get_hint();

            assert!(hint.is_some());

            assert!(word.contains(*hint.unwrap()));
        }
    }

    #[test]
    fn test_hint_returns_none_when_no_letter_to_guess() {
        let mut hangman = Hangman::new(&"t".to_string());

        hangman.guess_letter('t');

        let hint = hangman.get_hint();

        assert!(hint.is_none());
    }

    #[test]
    fn test_guess_word_all_blank() {
        let word = "hello".to_owned();
        let hangman = Hangman::new(&word);

        let guessed_word = hangman.get_guessed_word();

        assert_eq!(guessed_word, "_____");
    }

    #[test]
    fn test_guess_word_not_all_blank() {
        let word = "hello".to_owned();
        let mut hangman = Hangman::new(&word);

        hangman.guess_letter('l');

        let guessed_word = hangman.get_guessed_word();

        assert_eq!(guessed_word, "__ll_");
    }
}
