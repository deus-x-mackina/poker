use colored::Colorize;
use poker::{EvalClass, Evaluator};

mod common;

fn main() {
    let eval = Evaluator::new();

    println!("{}", WELCOME.bright_green().bold());

    let mut rl = common::editor();

    while let Ok(line) = rl.readline(PROMPT) {
        rl.add_history_entry(&line);
        match line.as_str() {
            "help" => println!("{}", HELP.bright_green().bold()),
            "quit" => break,
            _ => match poker::cards!(line).try_collect::<Box<_>>() {
                Ok(cards) => match eval.evaluate(cards) {
                    Ok(result) => {
                        let result_str = result.to_string();
                        let result_str = result_str.as_str();
                        println!(
                            "{}",
                            match result.class() {
                                EvalClass::HighCard { .. } => result_str.bright_red(),
                                EvalClass::Pair { .. } => result_str.bright_magenta(),
                                EvalClass::TwoPair { .. } => result_str.bright_purple(),
                                EvalClass::ThreeOfAKind { .. } => result_str.bright_yellow(),
                                EvalClass::Straight { .. } => result_str.bright_cyan(),
                                EvalClass::Flush { .. } => result_str.bright_blue(),
                                EvalClass::FullHouse { .. } => result_str.bright_white(),
                                EvalClass::FourOfAKind { .. } => result_str.bright_white().italic(),
                                EvalClass::StraightFlush { .. } =>
                                    result_str.bright_white().underline(),
                            }
                        );
                    }
                    Err(e) => println!("{}", e),
                },
                Err(e) => println!("{}", e),
            },
        }
    }
}

const PROMPT: &str = "poker> ";
const WELCOME: &str = r#"Welcome to the poker hand REPL!
For help, enter "help". To quit, enter "quit" or press CTRL+C."#;
const HELP: &str = r#"
Enter a list of 5 or more space-separated cards to be evaluated as a hand.
Cards are two character strings, where the first character represents
the card rank, and the second character represents the card suit.

Valid ranks: one of [23456789TJQKA]
Valid suits: one of [chsd]

Example: poker> Tc Jc Qc Kc Ac
"#;
