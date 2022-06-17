mod hangman;
mod hangman_view;
mod pics;

use cursive::views::{Dialog, TextView};

use crate::hangman_view::HangmanView;

fn main() {
    let mut siv = cursive::default();

    siv.add_layer(main_menu());

    siv.run();
}

fn main_menu() -> impl cursive::View {
    Dialog::around(TextView::new("Hangman Rust").center())
        .button("Start", |s| {
            s.pop_layer();
            s.add_layer(HangmanView::new());
        })
        .button("Quit", |s| s.quit())
}
