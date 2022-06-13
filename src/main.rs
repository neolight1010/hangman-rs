mod hangman;
mod pics;

use cursive::views::{Canvas, Dialog, LinearLayout, Panel, TextView};
use pics::HANGMAN_PICS;

use crate::hangman::Hangman;

fn main() {
    let mut siv = cursive::default();

    siv.add_layer(main_menu());

    siv.run();
}

fn main_menu() -> impl cursive::View {
    Dialog::around(TextView::new("Hangman Rust").center())
        .button("Start", |s| {
            s.pop_layer();
            s.add_layer(game_screen());
        })
        .button("Quit", |s| s.quit())
}

fn game_screen() -> impl cursive::View {
    let hangman = Hangman::new(&"hello".to_owned());

    Panel::new(
        LinearLayout::vertical()
            .child(TextView::new(hangman.get_pic()).center())
            .child(Canvas::new(()))
            .child(TextView::new(hangman.get_guessed_word()).center())
            .child(Canvas::new(()))
            .child(TextView::new("Press a key to guess a letter").center()),
    )
}
