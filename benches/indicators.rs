#[path = "../tests/fixtures/mod.rs"]
mod fixtures;

use crate::fixtures::load_reference_ohlcvs;

use criterion::{BatchSize, Criterion, Throughput, criterion_group, criterion_main};
use quantedge_ta::{Bb, BbConfig, Ema, EmaConfig, Sma, SmaConfig};
use std::{hint::black_box, num::NonZero};

fn nz(n: usize) -> NonZero<usize> {
    NonZero::new(n).expect("non zero value")
}

fn stream_benchmarks(c: &mut Criterion) {
    let bars = load_reference_ohlcvs();
    let mut group = c.benchmark_group("stream");
    group.throughput(Throughput::Elements(bars.len() as u64));

    group.bench_function("sma20", |b| {
        b.iter_batched(
            || Sma::new(SmaConfig::close(nz(20))),
            |mut ind| {
                for bar in &bars {
                    black_box(ind.compute(bar));
                }
            },
            BatchSize::PerIteration,
        );
    });

    group.bench_function("sma200", |b| {
        b.iter_batched(
            || Sma::new(SmaConfig::close(nz(200))),
            |mut ind| {
                for bar in &bars {
                    black_box(ind.compute(bar));
                }
            },
            BatchSize::PerIteration,
        );
    });

    group.bench_function("ema20", |b| {
        b.iter_batched(
            || Ema::new(EmaConfig::close(nz(20))),
            |mut ind| {
                for bar in &bars {
                    black_box(ind.compute(bar));
                }
            },
            BatchSize::PerIteration,
        );
    });

    group.bench_function("ema200", |b| {
        b.iter_batched(
            || Ema::new(EmaConfig::close(nz(200))),
            |mut ind| {
                for bar in &bars {
                    black_box(ind.compute(bar));
                }
            },
            BatchSize::PerIteration,
        );
    });

    group.bench_function("bb20", |b| {
        b.iter_batched(
            || Bb::new(BbConfig::close(nz(20))),
            |mut ind| {
                for bar in &bars {
                    black_box(ind.compute(bar));
                }
            },
            BatchSize::PerIteration,
        );
    });

    group.bench_function("bb200", |b| {
        b.iter_batched(
            || Bb::new(BbConfig::close(nz(200))),
            |mut ind| {
                for bar in &bars {
                    black_box(ind.compute(bar));
                }
            },
            BatchSize::PerIteration,
        );
    });

    group.finish();
}

fn tick_benchmarks(c: &mut Criterion) {
    let bars = load_reference_ohlcvs();
    let mut group = c.benchmark_group("tick");

    // Pre-feed all bars except the last, then benchmark a single compute() call.
    let (warmup, last) = bars.split_at(bars.len() - 1);

    macro_rules! tick_bench {
        ($name:expr, $ind_type:ty, $config:expr) => {
            group.bench_function($name, |b| {
                b.iter_batched(
                    || {
                        let mut ind = <$ind_type>::new($config);
                        for bar in warmup {
                            ind.compute(bar);
                        }
                        ind
                    },
                    |mut ind| {
                        black_box(ind.compute(&last[0]));
                    },
                    BatchSize::PerIteration,
                );
            });
        };
    }

    tick_bench!("sma20", Sma, SmaConfig::close(nz(20)));
    tick_bench!("sma200", Sma, SmaConfig::close(nz(200)));
    tick_bench!("ema20", Ema, EmaConfig::close(nz(20)));
    tick_bench!("ema200", Ema, EmaConfig::close(nz(200)));
    tick_bench!("bb20", Bb, BbConfig::close(nz(20)));
    tick_bench!("bb200", Bb, BbConfig::close(nz(200)));

    group.finish();
}

criterion_group!(benches, stream_benchmarks, tick_benchmarks);
criterion_main!(benches);
