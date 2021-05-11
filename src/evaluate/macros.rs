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
        use $crate::evaluate::{eval::Eval, utils};

        debug_assert_eq!($cards.len(), 5);

        let uniques = [
            $cards[0].unique_integer(),
            $cards[1].unique_integer(),
            $cards[2].unique_integer(),
            $cards[3].unique_integer(),
            $cards[4].unique_integer(),
        ];

        let detect_flush = uniques.iter().fold(0xF000, |acc, &x| acc & x) != 0;

        if detect_flush {
            let bit_rank_or = uniques.iter().fold(0, |acc, &x| acc | x) >> 16;
            let prime = utils::prime_product_from_rank_bits(bit_rank_or as i16);
            Eval($flush[&prime])
        } else {
            let prime = utils::prime_product_from_hand($cards);
            Eval($unsuited[&prime])
        }
    }};

    (@six_plus, $cards:expr, $closure:expr) => {{
        use $crate::evaluate::{eval::Eval, utils};

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
}
