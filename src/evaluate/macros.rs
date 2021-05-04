macro_rules! evaluation_impl {
    (@main, $cards:expr, $five:expr, $six:expr) => {{
        use $crate::error::EvalError;
        if $cards.all_unique() {
            match $cards.len() {
                x if x < 5 => Err(EvalError::InvalidHandSize(x)),
                5 => Ok($five),
                _ => Ok($six),
            }
        } else {
            Err(EvalError::CardsNotUnique($cards.to_vec()))
        }
    }};

    (@five, $cards:expr, $flush:expr, $unsuited:expr) => {{
        use $crate::evaluate::{utils, eval::Eval};

        debug_assert_eq!($cards.len(), 5);
        let detect_flush = $cards
            .iter()
            .fold(0xF000, |acc, card| acc & card.unique_integer())
            != 0;

        if detect_flush {
            let bit_rank_or = $cards
                .iter()
                .fold(0, |acc, card| acc | card.unique_integer())
                >> 16;
            let prime = utils::prime_product_from_rank_bits(bit_rank_or as i16);
            Eval($flush[&prime])
        } else {
            let prime = utils::prime_product_from_hand($cards);
            Eval($unsuited[&prime])
        }
    }};

    (@six_plus, $cards:expr, $closure:expr) => {{
        use $crate::evaluate::{utils, eval::Eval};

        debug_assert!($cards.len() > 5);
        let mut current_max = Eval::WORST;
        let all_five_card_combos = utils::combinations_generator($cards.iter().copied(), 5);
        for combo in all_five_card_combos {
            let score = $closure(&combo);
            if score > current_max {
                current_max = score;
            }
        }
        current_max
    }};

    (@six_plus_dyn, $this:expr, $f:ident, $combos:expr) => {
        $this.$f(&$combos)
    };
}
