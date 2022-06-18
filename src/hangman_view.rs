use cursive::{
    event::{Event, EventResult},
    traits::{Finder, Nameable},
    view::{IntoBoxedView, Selector, ViewWrapper},
    views::{BoxedView, Canvas, LinearLayout, Panel, TextView, DummyView},
    wrap_impl, View,
};

use crate::hangman::{GameState, Hangman};
pub struct HangmanView<T: View> {
    hangman: Hangman,
    view: T,
}

impl HangmanView<BoxedView> {
    pub fn new() -> Self {
        let hangman = Hangman::new("hello");

        let view = match hangman.get_game_state() {
            GameState::Playing => Self::playing_view(&hangman),
            _ => Self::game_over_view(&hangman),
        };

        Self {
            hangman,
            view: BoxedView::new(view),
        }
    }

    fn playing_view(hangman: &Hangman) -> Box<dyn View> {
        Panel::new(
            LinearLayout::vertical()
                .child(TextView::new(hangman.get_pic()).center().with_name("pic"))
                .child(DummyView)
                .child(
                    TextView::new(hangman.get_guessed_word())
                        .center()
                        .with_name("guessed_word"),
                )
                .child(DummyView)
                .child(TextView::new("Press a key to guess a letter").center()),
        )
        .into_boxed_view()
    }

    fn game_over_view(hangman: &Hangman) -> Box<dyn View> {
        Panel::new(
            LinearLayout::vertical()
                .child(TextView::new(hangman.get_pic()).center())
                .child(DummyView)
                .child(TextView::new("Game Over :c").center()),
        )
        .into_boxed_view()
    }

    fn game_win_view(hangman: &Hangman) -> Box<dyn View> {
        Panel::new(
            LinearLayout::vertical()
                .child(TextView::new(hangman.get_pic()).center())
                .child(DummyView)
                .child(TextView::new("You Won! :D").center()),
        )
        .into_boxed_view()
    }
}

impl ViewWrapper for HangmanView<BoxedView> {
    wrap_impl!(self.view: BoxedView);

    fn wrap_on_event(&mut self, event: Event) -> EventResult {
        if self.hangman.get_game_state() != GameState::Playing {
            return EventResult::Ignored;
        }

        match event {
            Event::Char(c) => {
                self.hangman.guess_letter(c);

                let pic = self.hangman.get_pic().to_owned();
                let guessed_word = self.hangman.get_guessed_word().to_owned();

                self.call_on(&Selector::Name("pic"), |v: &mut TextView| {
                    v.set_content(pic);
                });

                self.call_on(&Selector::Name("guessed_word"), |v: &mut TextView| {
                    v.set_content(guessed_word);
                });

                match self.hangman.get_game_state() {
                    GameState::Lose => {
                        self.view = BoxedView::new(HangmanView::game_over_view(&self.hangman));
                    }
                    GameState::Win => {
                        self.view = BoxedView::new(HangmanView::game_win_view(&self.hangman));
                    }
                    _ => (),
                };
            }
            _ => (),
        }

        EventResult::Consumed(None)
    }
}
