# `poker`: The Poker Evaluation Crate

[![Crates.io](https://img.shields.io/crates/v/poker)](https://crates.io/crates/poker)
[![Docs.rs](https://docs.rs/poker/badge.svg)](https://docs.rs/poker)
[![unsafe forbidden](https://img.shields.io/badge/unsafe-forbidden-success.svg)](https://github.com/rust-secure-code/safety-dance/)

`poker` is a Rust crate for the speedy evaluation and comparison of poker hands.
It supports both 5-card and 3-card poker hand evaluation. The crate is based on the 
[`treys`](https://github.com/ihendley/treys) Python package and the algorithms found 
within, with mild adaptations and some personal touches to try to make it as 
idiomatic as possible in Rust.

```rust
use poker::{Evaluator, deck, Card};

fn main() {
    // Create a hand evaluator
    let eval = Evaluator::new();

    // Generate a shuffled deck
    let mut deck = deck::shuffled();

    // Deal a 5-card hand
    let hand: Vec<Card> = deck.drain(..5).collect();

    // Evaluate 5-card hand
    let result = eval.evaluate_five(&hand).expect("Couldn't evaluate hand!");

    // Print the hand result
    println!("5-card hand: {}", result);

    // Deal a 3-card hand
    let three_card_hand: Vec<Card> = deck.drain(..3).collect();

    // Evaluate 3-card hand
    let three_result = eval.evaluate_three(&three_card_hand).expect("Couldn't evaluate hand!");

    // Print the 3-card hand result
    println!("3-card hand: {}", three_result);

    let other_hand = deck.drain(..5);

    let other_result = eval
        .evaluate_five(&other_hand)
        .expect("Couldn't evaluate hand!");

    if result.is_better_than(other_result) {
        println!("You win!");
    } else {
        println!("You lose...");
    }
}
```

## Using `poker`

Add poker to the `dependencies` in your `Cargo.toml` file:

```toml
[dependencies]
poker = "0.7"
```

## Features

`poker` supports both 5-card and 3-card poker hand evaluation with fast lookup tables.

The crate currently has two features:

1. **`rand`** (enabled by default): Depends on the [`rand`](https://crates.io/crates/rand)
   crate for shuffling generated decks.

2. **`static_lookup`** (not enabled by default): Opens up the `poker::evaluate::static_lookup` module,
   which contains the free `evaluate` function. It works similar to
   `Evaluator::evaluate`, but semantically it uses a static data structure that
   does not rely on heap allocations. Behind the scenes, the crate downloads data
   from [another repository](https://github.com/deus-x-mackina/poker-lookup-table)
   at build time and therefore won't have to construct this deterministic data at runtime.

```toml
[dependencies]
# To use without `rand`, add `default-features = false`
poker = { version = "0.7", features = ["static_lookup"] }
```

## Examples

`poker` includes two fun built-in examples: `poker-repl` and `jacks-or-better`.
`poker-repl` is a `repl`-like environment where you can evaluate different poker
hands. `jacks-or-better` is a terminal re-creation of the Jacks or Better video
poker game. Rules for the game can be found
[here](https://www.liveabout.com/jacks-or-better-video-poker-2727991), with the
following payout chart:

| 5-card hand     | Payout (bet multiple) |
| --------------- | --------------------- |
| Royal Flush     | 4000                  |
| Straight Flush  | 250                   |
| Four of a Kind  | 25                    |
| Full House      | 9                     |
| Flush           | 6                     |
| Straight        | 4                     |
| Three of a Kind | 3                     |
| Two Pair        | 2                     |
| Jacks or Better | 1                     |

> ### DISCLAIMER
>
> The `jacks-or-better` example from the `poker` crate has themes of gambling
> using a currency called `credits`. This program is meant for example purposes
> only to illustrate one possible use of this library. There is no risk
> associated with running the example as it can be terminated and restarted at
> any time.
>
> Please be aware of the financial risk of real gambling.

You can install these examples through `cargo` by running the following command:

```bash
cargo install poker --example=poker-repl
cargo install poker --example=jacks-or-better
# Then you can run the programs, assuming they were installed somewhere in $PATH
poker-repl
jacks-or-better
```

You can also run the examples through a cloned git repository.

```bash
git clone https://github.com/deus-x-mackina/poker.git
cd poker
cargo run --example=poker-repl
cargo run --example=jacks-or-better
```

## License

Licensed under the MIT license ([LICENSE.txt](LICENSE.txt) or
<http://opensource.org/licenses/MIT>).
