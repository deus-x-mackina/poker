# `poker`: The Poker Evaluation Crate

[![Crates.io](https://img.shields.io/crates/v/poker)](https://crates.io/crates/poker)
[![Docs.rs](https://docs.rs/poker/badge.svg)](https://docs.rs/poker)
[![unsafe forbidden](https://img.shields.io/badge/unsafe-forbidden-success.svg)](https://github.com/rust-secure-code/safety-dance/)

`poker` is a Rust crate for the speedy evaluation and comparison of poker hands.
It it based on the [`treys`](https://github.com/ihendley/treys) Python package
and the algorithms found within, with mild adaptations and some personal touches
to try to make it as idiomatic as possible in Rust.

```rust
use poker::{Evaluator, cards, Card};

fn main() {
    // Create a hand evaluator
    let eval = Evaluator::new();

    // Generate a shuffled deck
    let mut deck = Card::generate_shuffled_deck();

    // Deal a hand
    let hand: Vec<Card> = deck.drain(..5).collect();

    // Evaluate
    let result = eval.evaluate(hand).expect("Couldn't evaluate hand!");

    // Print the hand result
    println!("{}", result);
}
```

## Using `poker`

Add poker to the `dependencies` in your `Cargo.toml` file:

```toml
[dependencies]
poker = "0.5"
```

## Features

`poker` currently has two features. One depends on the
[`rand`](https://crates.io/crates/rand) crate, in order to shuffle generated
decks. This is enabled by default.

The second feature, which is also not enabled by default is `static_lookup`.
Enabling this feature opens up the `poker::evaluate::static_lookup` module,
which contains the free `evaluate` function. It works similar to
`Evaluator::evaluate`, but semantically it uses a static data structure that
does not rely on heap allocations. Behind the scenes, the crate downloads data
from [another repository](https://github.com/deus-x-mackina/poker-lookup-table)
at build time and therefore won't have to construct this deterministic data at runtime.

```toml
[dependencies]
# To use without `rand`, add `default-features = false`
poker = { version = "0.5", features = ["static_lookup"] }
```

## A Note on Performance

In order to ensure `rustc` can make appropriate inlining and optimization decisions,
remember to use link-time optimization in your release builds. This comes at the cost
of slower compilation times. In your `Cargo.toml`:

```toml
[profile.release]
# ...
lto = true # the default is false!
```

## Examples

`poker` includes two fun builtin examples: `poker-repl` and `jacks-or-better`.
`poker-repl` is a `repl`-like environment when you can evaluate different poker
hands. `jacks-or-better` is a terminal re-creation of the Jacks or Better video
poker game. Rules for the game can be found
[here](https://www.liveabout.com/jacks-or-better-video-poker-2727991), with the
following payout chart:

| 5-card hand     | Payout (bet multiple) |
|-----------------|-----------------------|
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
