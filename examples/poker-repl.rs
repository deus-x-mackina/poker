use colored::Colorize;
use poker::Evaluator;

mod common;

fn main() {
    let eval = Evaluator::new();

    println!("{}", WELCOME.bright_green().bold());

    let mut rl = common::editor();

    while let Ok(line) = rl.readline(PROMPT) {
        rl.add_history_entry(&line).unwrap();
        match line.as_str() {
            "help" => println!("{}", HELP.bright_green().bold()),
            "quit" => break,
            _ => match poker::cards!(line).try_collect::<Box<_>>() {
                Ok(cards) if cards.len() >= 5 => match eval.evaluate_five(cards) {
                    Ok(result) => {
                        let result_str = result.to_string();
                        let result_str = result_str.as_str();
                        println!(
                            "{}",
                            match result {
                                result if result.is_high_card() => result_str.bright_red(),
                                result if result.is_pair() => result_str.bright_magenta(),
                                result if result.is_two_pair() => result_str.bright_purple(),
                                result if result.is_three_of_a_kind() => result_str.bright_yellow(),
                                result if result.is_straight() => result_str.bright_cyan(),
                                result if result.is_flush() => result_str.bright_blue(),
                                result if result.is_full_house() => result_str.bright_white(),
                                result if result.is_four_of_a_kind() =>
                                    result_str.bright_white().italic(),
                                result if result.is_straight_flush() =>
                                    result_str.bright_white().underline(),
                                _ => unreachable!(),
                            }
                        );
                    }
                    Err(e) => println!("{e}"),
                },
                Ok(cards) if cards.len() == 3 => match eval.evaluate_three(cards) {
                    Ok(result) => {
                        let result_str = result.to_string();
                        let result_str = result_str.as_str();
                        println!(
                            "{}",
                            match result {
                                result if result.is_high_card() => result_str.bright_red(),
                                result if result.is_pair() => result_str.bright_magenta(),
                                result if result.is_three_of_a_kind() => result_str.bright_yellow(),
                                result if result.is_straight() => result_str.bright_cyan(),
                                result if result.is_flush() => result_str.bright_blue(),
                                result if result.is_straight_flush() =>
                                    result_str.bright_white().underline(),
                                _ => unreachable!(),
                            }
                        );
                    }
                    Err(e) => println!("{e}"),
                },
                Ok(cards) => println!(
                    "Can't evaluate a set of {} card{}. Please enter 3 or 5+ cards to evaluate.",
                    cards.len(),
                    if cards.len() == 1 { "" } else { "s" }
                ),
                Err(e) => println!("{e}"),
            },
        }
    }
}

const PROMPT: &str = "poker> ";
const WELCOME: &str = r#"Welcome to the poker hand REPL!
For help, enter "help". To quit, enter "quit" or press CTRL+C."#;
const HELP: &str = r#"
Enter a list of 3 or 5+ space-separated cards to be evaluated as a hand.
Cards are two character strings, where the first character represents
the card rank, and the second character represents the card suit.

Valid ranks: one of [23456789TJQKA]
Valid suits: one of [chsd]

Example: poker> Tc Jc Qc Kc Ac
"#;
