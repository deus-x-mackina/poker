use std::{fs::File, io::{Write, BufWriter}, path::Path};

use poker::Evaluator;

fn main() {
    let evaluator = Evaluator::new();
    let table = evaluator.0;
    let flush_lookup = table.flush_lookup;
    let unsuited_lookup = table.unsuited_lookup;

    let mut flush_builder = phf_codegen::Map::new();
    for (k, v) in flush_lookup {
        flush_builder.entry(k, &format!("Meta::{:?}", v));
    }

    let mut unsuited_builder = phf_codegen::Map::new();
    for (k, v) in unsuited_lookup {
        unsuited_builder.entry(k, &format!("Meta::{:?}", v));
    }

    let path = Path::new("src/evaluate/cached.rs");
    let mut file = BufWriter::new(File::create(&path).unwrap());

    writeln!(
        file,
        "\
use super::meta::Meta;
use crate::card::rank::Rank::*;

static FLUSH_LOOKUP: ::phf::Map<i32, Meta> = {};

static UNSUITED_LOOKUP: ::phf::Map<i32, Meta> = {};
        ",
        flush_builder.build(),
        unsuited_builder.build()
     ).unwrap();
}
