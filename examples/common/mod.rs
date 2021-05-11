//! Functionality shared across examples.

use std::borrow::Cow;

use colored::Colorize;
use rustyline::{highlight::Highlighter, ColorMode, Config, Editor as RlEditor};
use rustyline_derive::{Completer, Helper, Hinter, Validator};

#[derive(Completer, Helper, Hinter, Validator)]
pub struct ColorPrompt;

impl Highlighter for ColorPrompt {
    fn highlight_prompt<'b, 's: 'b, 'p: 'b>(
        &'s self,
        prompt: &'p str,
        default: bool,
    ) -> Cow<'b, str> {
        if default {
            Cow::Owned(prompt.bright_green().to_string())
        } else {
            Cow::Borrowed(prompt)
        }
    }
}

pub fn editor() -> Editor {
    let config = Config::builder().color_mode(ColorMode::Enabled).build();
    let mut rl = Editor::with_config(config);
    rl.set_helper(Some(ColorPrompt));
    rl
}

pub type Editor = RlEditor<ColorPrompt>;

#[allow(unused)]
macro_rules! clear_terminal {
    () => {
        print!("\x1B[2J\x1B[1;1H")
    };
}
