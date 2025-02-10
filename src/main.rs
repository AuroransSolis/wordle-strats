#![feature(
    generic_const_exprs,
    iter_intersperse,
    portable_simd,
    specialization,
    str_from_raw_parts
)]
#![allow(incomplete_features)]

mod comb;
mod utils;
mod words;

use comb::IterComb;
use rayon::{iter::ParallelBridge, prelude::ParallelIterator};
use std::{cmp::Ordering, time::Instant};
use utils::{strat_strings, word_passes_filters, Aggregate, AnalysisResult};
use words::{Word, COMBINED_WORDS, POSSIBLE_WORDS, TOTAL_STRATS, TOTAL_STRATS_LEN};

pub const DEPTH: usize = 2;

fn main() {
    let start = Instant::now();
    let mut results = COMBINED_WORDS
        .iter_comb::<DEPTH>()
        .map(|strat| strat.map(|&word| word))
        .enumerate()
        .par_bridge()
        .inspect(|(i, strat)| {
            eprintln!(
                "checking strat: {:?} | {i:>w$} of {:>w$}",
                strat_strings(strat),
                TOTAL_STRATS,
                w = TOTAL_STRATS_LEN,
            )
        })
        .map(|(_, strat): (usize, [Word; DEPTH])| {
            let mut agg = Aggregate::new();
            for secret in POSSIBLE_WORDS {
                let filters = strat.map(|guess| utils::guess_to_filters(secret, guess));
                let mut j = 0;
                'outer: for word in POSSIBLE_WORDS {
                    for filter in filters {
                        if !word_passes_filters(word, filter) {
                            continue 'outer;
                        }
                    }
                    j += 1;
                }
                agg.update(j as f64);
            }
            let [avg, std] = agg.finalise();
            (strat, AnalysisResult { avg, std })
        })
        .collect::<Vec<([Word; DEPTH], AnalysisResult)>>();
    let elapsed = start.elapsed();
    results.sort_by(|(_, pa), (_, pb)| {
        pa.avg
            .partial_cmp(&pb.avg)
            .or(pa.std.partial_cmp(&pb.std))
            .unwrap_or(Ordering::Equal)
    });
    let strat_string_len = DEPTH * 5 + (DEPTH - 1) * 3;
    println!(
        "{0:>width0$}rank: {0:>width1$}strat,     avg,     std",
        "",
        width0 = TOTAL_STRATS_LEN - 4,
        width1 = strat_string_len - 5,
    );
    results
        .into_iter()
        .enumerate()
        .for_each(|(place, (strat, ar))| {
            let AnalysisResult { avg, std } = ar;
            let mut strat_string = String::with_capacity(strat_string_len);
            for item in strat_strings(&strat).into_iter().intersperse(" + ") {
                strat_string.push_str(item);
            }
            println!("{:>5}: {}, {avg:>7.3}, {std:>7.3}", place + 1, strat_string,);
        });
    println!("took: {elapsed:?}");
}
