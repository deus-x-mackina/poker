use std::borrow::Cow;

use colored::Colorize;
use rustyline::highlight::Highlighter;
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
