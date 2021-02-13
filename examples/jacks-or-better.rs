use colored::*;
use example_helpers::ColorPrompt;
use poker::{Card, EvalClass, Evaluator, Rank};
use rand::prelude::*;
use rustyline::{ColorMode, Config, Editor};

fn main() {
    // Clear the screen
    print!("\x1B[2J\x1B[1;1H");
    let mut rng = thread_rng();
    let eval = Evaluator::new();

    println!("{}", WELCOME.bright_green().bold());

    let config = Config::builder().color_mode(ColorMode::Enabled).build();
    let mut rl = Editor::with_config(config);
    rl.set_helper(Some(ColorPrompt));

    // Game setup
    let mut deck = Card::generate_shuffled_deck().to_vec();
    let mut credits = STARTING_CREDITS;
    let mut hand = Vec::with_capacity(5);

    'game: loop {
        // Get wager
        let wager = 'wager: loop {
            let message = format!("Enter a wager. (Credits: {}, max: {})", credits, MAX_WAGER);
            println!("{}", message.as_str().bright_green().bold());
            if let Ok(wager) = rl.readline(PROMPT) {
                if wager == "quit" {
                    break 'game;
                } else if let Ok(wager) = wager.parse::<usize>() {
                    if wager <= MAX_WAGER {
                        if wager > credits {
                            println!(
                                "Not enough credits! Got {} but you have {}.",
                                wager, credits
                            );
                            continue 'wager;
                        } else {
                            break 'wager wager;
                        }
                    } else {
                        println!(
                            "Invalid wager amount '{}'. Expected amount from 1 to {}",
                            wager, MAX_WAGER
                        );
                        continue 'wager;
                    }
                } else {
                    println!(
                        "Invalid input '{}'. Expected wager from 1 to {}",
                        wager, MAX_WAGER
                    );
                    continue 'wager;
                }
            } else {
                break 'game;
            }
        };
        credits -= wager;

        // Clear the screen
        print!("\x1B[2J\x1B[1;1H");

        // Deal hand and print cards, along with helper numbers
        hand.extend(deck.drain(0..5));
        let first_eval = eval.evaluate(&hand).unwrap();
        println!(
            "{} {} {} {} {} ({})\n  (1)    (2)    (3)    (4)    (5)",
            &hand[0], &hand[1], &hand[2], &hand[3], &hand[4], first_eval
        );

        // Get swaps, if any
        let swaps: Vec<usize> = 'swaps: loop {
            let message = "Enter the cards' numbers you wish to swap, if any.";
            println!("{}", message.bright_green().bold());
            if let Ok(input) = rl.readline(PROMPT) {
                if input == "quit" { break 'game }
                let parsed = input
                    .split_whitespace()
                    .map(str::parse)
                    .collect::<Result<Vec<_>, _>>();
                if let Ok(swaps) = parsed {
                    if swaps.iter().all(|num| matches!(*num, 1..=5)) {
                        if matches!(swaps.len(), 0..=5) {
                            break 'swaps swaps.into_iter().map(|i| i - 1).collect();
                        } else {
                            println!(
                                "Error parsing input '{}'. Did not specify between 0 and 5 \
                                 numbers.",
                                input
                            );
                            continue 'swaps;
                        }
                    } else {
                        println!(
                            "Error parsing input '{}'. Not all listed numbers are between 1 and 5.",
                            input,
                        );
                        continue 'swaps;
                    }
                } else {
                    println!(
                        "Error parsing input '{}'. Expected space-separated list of numbers from \
                         1 to 5.",
                        input
                    );
                    continue 'swaps;
                }
            } else {
                break 'game;
            }
        };

        // Replace swaps in hand
        if !swaps.is_empty() {
            for &index in &swaps {
                deck.push(hand[index]);
            }
            deck.shuffle(&mut rng);
            for (i, dealt_card) in deck.drain(0..swaps.len()).enumerate() {
                hand[swaps[i]] = dealt_card;
            }
        }

        // Print second hand
        let second_eval = eval.evaluate(&hand).unwrap();
        println!(
            "{} {} {} {} {} ({})",
            &hand[0], &hand[1], &hand[2], &hand[3], &hand[4], second_eval
        );

        // Compare hands
        match (second_eval, first_eval) {
            (sec, fir) if sec > fir => {
                let message = format!(
                    "You got a better hand!{}",
                    if second_eval.class() == first_eval.class() {
                        " (kicker)"
                    } else {
                        ""
                    }
                );
                println!("{}", message.as_str().bright_green().bold());
            }
            (sec, fir) if sec < fir => {
                let message = format!(
                    "You got a worse hand...{}",
                    if second_eval.class() == first_eval.class() {
                        " (kicker)"
                    } else {
                        ""
                    }
                );
                println!("{}", message.as_str().bright_red().bold());
            }
            // print nothing if nothing was swapped
            _ => {}
        }

        // Calculate winnings
        let winnings = payout(wager, second_eval.class());
        let winnings_string = format!("Winnings: {}", winnings);
        println!(
            "{}",
            if winnings > 0 {
                winnings_string.as_str().bright_green().bold()
            } else {
                winnings_string.as_str().bright_red().bold()
            }
        );
        credits += winnings;
        if credits == 0 {
            println!("{}", "Game over!".bright_red().bold());
            break 'game;
        }

        // Cleanup
        println!();
        deck.extend(hand.drain(..));
        deck.shuffle(&mut rng);

        // Loop back
    }
}

const STARTING_CREDITS: usize = 100;
const MAX_WAGER: usize = 5;

pub const fn payout(wager: usize, class: EvalClass) -> usize {
    use EvalClass::*;
    match class {
        // I would use a `if pair >= Rank::Jack` guard here, but then the function wouldn't be
        // `const`!
        Pair { pair: Rank::Jack }
        | Pair { pair: Rank::Queen }
        | Pair { pair: Rank::King }
        | Pair { pair: Rank::Ace } => wager,

        TwoPair { .. } => 2 * wager,
        ThreeOfAKind { .. } => 3 * wager,
        Straight { .. } => 4 * wager,
        Flush { .. } => 6 * wager,
        FullHouse { .. } => 9 * wager,
        FourOfAKind { .. } => 25 * wager,

        // Royal flush
        StraightFlush {
            high_rank: Rank::Ace,
        } => {
            if wager == MAX_WAGER {
                4000
            } else {
                250 * wager
            }
        }
        StraightFlush { .. } => 50 * wager,

        _ => 0,
    }
}

const PROMPT: &str = ">>> ";

const WELCOME: &str = r#"Welcome to Jacks or Better video poker!
To quit, enter "quit" or press CTRL+C.
"#;
