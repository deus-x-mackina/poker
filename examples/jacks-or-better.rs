use std::collections::HashSet;

use colored::Colorize;
use example_helpers::ColorPrompt;
use itertools::Itertools;
use poker::{Card, EvalClass, Evaluator, Rank};
use rand::prelude::*;
use rustyline::{ColorMode, Config, Editor};

const STARTING_CREDITS: usize = 100;
const MAX_WAGER: usize = 5;
const PROMPT: &str = ">>> ";
const WELCOME: &str = r#"Welcome to Jacks or Better video poker!
To quit, enter "quit" or press CTRL+C.
"#;

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
        let wager = match get_wager(&mut rl, credits) {
            None => break 'game,
            Some(wager) => wager,
        };
        credits -= wager;

        // Deal hand and print cards, along with helper numbers
        hand.extend(deck.drain(0..5));
        let first_eval = eval.evaluate(&hand).unwrap();
        println!(
            "\x1B[2J\x1B[1;1H{} {} {} {} {} ({})\n(1)    (2)    (3)    (4)    (5)",
            &hand[0], &hand[1], &hand[2], &hand[3], &hand[4], first_eval
        );

        // Get swaps as a vector of indices in the hand the user wishes to swap
        let swaps: Vec<usize> = match get_swaps(&mut rl) {
            None => break 'game,
            Some(swaps) => swaps,
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

        // Compare swapped hand to first hand
        match (second_eval.class(), first_eval.class()) {
            (sec, fir) if sec > fir => {
                println!("{}", "You got a better hand!".bright_green().bold())
            }
            (sec, fir) if sec < fir => {
                println!("{}", "You got a worse hand...".bright_red().bold())
            }
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

/// Attempt to read a wager from stdin. Returns None if the outer 'game loop
/// needs to be broken.
#[inline]
fn get_wager(rl: &mut Editor<ColorPrompt>, credits: usize) -> Option<usize> {
    let message = format!("Enter a wager. (Credits: {}, max: {})", credits, MAX_WAGER);
    loop {
        println!("{}", message.as_str().bright_green().bold());

        // Try to read a usize from stdin
        let initial_wager = match rl.readline(PROMPT) {
            Err(_) => return None,
            Ok(input) if input == "quit" => return None,

            Ok(wager) => {
                if let Ok(wager) = wager.parse::<usize>() {
                    wager
                } else {
                    println!(
                        "Invalid input '{}'. Expected wager from 1 to {}.\n",
                        wager, MAX_WAGER
                    );
                    continue;
                }
            }
        };

        // Validate the wager
        match initial_wager {
            // Can't bet 0 credits
            0 => println!("Sorry, you can't bet 0 credits!\n"),

            // Can't bet over max
            x if x > MAX_WAGER => println!(
                "Invalid wager amount '{}'. Expected amount from 1 to {}.\n",
                x, MAX_WAGER
            ),

            // Can't bet more than you have
            x if x > credits => {
                println!("Not enough credits! Got {} but you have {}.\n", x, credits)
            }

            // Good to go!
            x => return Some(x),
        }
    }
}

/// Attempt to read swaps from stdin. Return None if the outer 'game loop needs
/// to be broken.
#[inline]
fn get_swaps(rl: &mut Editor<ColorPrompt>) -> Option<Vec<usize>> {
    let message = "Enter the cards' numbers you wish to swap, if any.";
    loop {
        println!("{}", message.bright_green().bold());

        // Get input from stdin
        let input = match rl.readline(PROMPT) {
            Err(_) => return None,
            Ok(input) if input == "quit" => return None,
            Ok(input) => input,
        };

        // Parse a space-separated list of numbers
        let parsed: Result<Vec<usize>, _> = input.split_whitespace().map(str::parse).collect();

        let swaps = match parsed {
            Err(_) => {
                println!(
                    "Error parsing input '{}'. Expected space-separated list of card numbers to \
                     swap.\n",
                    input,
                );
                continue;
            }
            Ok(swaps) => swaps,
        };

        // Validate the indices
        let mut duplicates = HashSet::with_capacity(swaps.len());
        match swaps {
            // All indices should be in the ranger 1..=5
            swaps if !swaps.iter().all(|num| matches!(*num, 1..=5)) => println!(
                "Error parsing input '{}'. Not all listed numbers are between 1 and 5.\n",
                input,
            ),

            // Only accept 0 to 5 swaps
            swaps if !matches!(swaps.len(), 0..=5) => println!(
                "Error parsing input '{}'. Specified more than 5 cards to swap.\n",
                input
            ),

            // No duplicate swaps
            swaps if !swaps.iter().all(|x| duplicates.insert(*x)) => {
                let counts = swaps
                    .into_iter()
                    .counts()
                    .into_iter()
                    .filter_map(|(swap, count)| {
                        if count > 1 {
                            Some(swap.to_string())
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>();
                println!(
                    "Cannot provide duplicate cards to swap: {}.\n",
                    counts.join(" ")
                )
            }

            // Good to go!
            swaps => return Some(swaps.into_iter().map(|x| x - 1).collect()),
        }
    }
}

const fn payout(wager: usize, class: EvalClass) -> usize {
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
