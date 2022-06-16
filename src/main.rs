mod hangman;
mod pics;

use cursive::{
    event::{Event, EventTrigger},
    views::{Canvas, Dialog, LinearLayout, OnEventView, Panel, TextView},
};

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

    let panel = Panel::new(
        LinearLayout::vertical()
            .child(TextView::new(hangman.get_pic()).center())
            .child(Canvas::new(()))
            .child(TextView::new(hangman.get_guessed_word()).center())
            .child(Canvas::new(()))
            .child(TextView::new("Press a key to guess a letter").center()),
    );

    OnEventView::new(panel).on_pre_event_inner(EventTrigger::any(), |_, event| {
        match event {
            Event::Char(c) => println!("{c}"),
            _ => (),
        }

        None
    })
}
