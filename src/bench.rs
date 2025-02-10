#![feature(
    generic_const_exprs,
    iter_intersperse,
    nonzero_ops,
    portable_simd,
    str_from_raw_parts
)]
#![allow(incomplete_features)]

use criterion::{black_box, criterion_group, criterion_main, BatchSize, Criterion};
use rand::{
    rngs::StdRng,
    seq::IndexedRandom,
    SeedableRng,
};
use utils::{guess_to_filters, word_passes_filters, Aggregate};
use words::POSSIBLE_WORDS;

use crate::utils::AnalysisResult;

#[allow(dead_code)]
mod utils;
mod words;

const DEPTH: usize = 3;

fn make_filters(c: &mut Criterion) {
    let mut rng = StdRng::from_os_rng();
    c.bench_function("make filters", |b| {
        b.iter_batched(
            || {
                let strat = POSSIBLE_WORDS
                    .choose_multiple_array::<StdRng, DEPTH>(&mut rng)
                    .unwrap();
                let secret = *POSSIBLE_WORDS.choose(&mut rng).unwrap();
                (strat, secret)
            },
            |(strat, secret)| {
                black_box(strat.map(|word| guess_to_filters(secret, word)));
            },
            BatchSize::SmallInput,
        )
    });
}

fn apply_filter(c: &mut Criterion) {
    let mut rng = StdRng::from_os_rng();
    c.bench_function("apply filter", |b| {
        b.iter_batched(
            || {
                let [word, secret] = POSSIBLE_WORDS
                    .choose_multiple_array::<StdRng, 2>(&mut rng)
                    .unwrap();
                let filter = guess_to_filters(secret, word);
                let randword = *POSSIBLE_WORDS.choose(&mut rng).unwrap();
                (filter, randword)
            },
            |(filter, word)| {
                black_box(word_passes_filters(word, filter));
            },
            BatchSize::SmallInput,
        )
    });
}

fn apply_filters(c: &mut Criterion) {
    let mut rng = StdRng::from_os_rng();
    c.bench_function("apply filters", |b| {
        b.iter_batched(
            || {
                let strat = POSSIBLE_WORDS
                    .choose_multiple_array::<StdRng, DEPTH>(&mut rng)
                    .unwrap();
                let secret = *POSSIBLE_WORDS.choose(&mut rng).unwrap();
                let filters = strat.map(|word| guess_to_filters(secret, word));
                let randword = *POSSIBLE_WORDS.choose(&mut rng).unwrap();
                (filters, randword)
            },
            |(filters, randword)| {
                for filter in filters {
                    if !word_passes_filters(randword, filter) {
                        break;
                    }
                }
            },
            BatchSize::SmallInput,
        )
    });
}

fn poss_remaining(c: &mut Criterion) {
    let mut rng = StdRng::from_os_rng();
    c.bench_function("possible remaining", |b| {
        b.iter_batched(
            || {
                let strat = POSSIBLE_WORDS
                    .choose_multiple_array::<StdRng, DEPTH>(&mut rng)
                    .unwrap();
                let secret = *POSSIBLE_WORDS.choose(&mut rng).unwrap();
                (strat, secret)
            },
            |(strat, secret)| {
                let filters = strat.map(|word| guess_to_filters(secret, word));
                let mut poss_remaining = 0;
                'outer: for word in POSSIBLE_WORDS {
                    for filter in filters {
                        if !word_passes_filters(word, filter) {
                            continue 'outer;
                        }
                    }
                    poss_remaining += 1;
                }
                black_box(poss_remaining)
            },
            BatchSize::SmallInput,
        )
    });
}

fn whole_analysis(c: &mut Criterion) {
    let mut rng = StdRng::from_os_rng();
    c.bench_function("whole analysis", |b| {
        b.iter_batched(
            || {
                POSSIBLE_WORDS
                    .choose_multiple_array::<StdRng, DEPTH>(&mut rng)
                    .unwrap()
            },
            |strat| {
                let mut agg = Aggregate::new();
                for secret in POSSIBLE_WORDS {
                    let filters = strat.map(|word| guess_to_filters(secret, word));
                    let mut poss_remaining = 0;
                    'outer: for word in POSSIBLE_WORDS {
                        for filter in filters {
                            if !word_passes_filters(word, filter) {
                                continue 'outer;
                            }
                        }
                        poss_remaining += 1;
                    }
                    agg.update(poss_remaining as f64);
                }
                let [avg, std] = agg.finalise();
                black_box((
                    strat,
                    AnalysisResult {
                        avg,
                        std,
                    },
                ))
            },
            BatchSize::SmallInput,
        )
    });
}

criterion_group!(
    benches,
    make_filters,
    apply_filter,
    apply_filters,
    poss_remaining,
    whole_analysis,
);
criterion_main!(benches);
