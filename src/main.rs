mod hangman;

use cursive::views::{Dialog, TextView};

use crate::hangman::Hangman;

fn main() {
    let hangman = Hangman::new(&"hello".to_string());

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
    TextView::new("My game screen!")
}
