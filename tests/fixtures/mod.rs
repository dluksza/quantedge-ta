#![allow(dead_code)]

use quantedge_ta::{Ohlcv, Price, Timestamp};
use serde::{Deserialize, de::DeserializeOwned};

/// OHLCV bar parsed from Binance CSV.
#[derive(Debug, Clone, Deserialize)]
pub struct RefBar {
    pub open_time: u64,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: f64,
}

impl Ohlcv for RefBar {
    fn open(&self) -> Price {
        self.open
    }

    fn high(&self) -> Price {
        self.high
    }

    fn low(&self) -> Price {
        self.low
    }

    fn close(&self) -> Price {
        self.close
    }

    fn open_time(&self) -> Timestamp {
        self.open_time
    }

    fn volume(&self) -> f64 {
        self.volume
    }
}

/// Reference value with timestamp.
#[derive(Debug, Deserialize)]
pub struct RefValue {
    pub open_time: u64,
    pub expected: f64,
}

/// Reference BB value with timestamp.
#[derive(Debug, Deserialize)]
pub struct RefBbValue {
    pub open_time: u64,
    pub upper: f64,
    pub middle: f64,
    pub lower: f64,
}

const OHLCV_PATH: &str = "tests/fixtures/data/btcusdt-1h.csv";

/// Load reference OHLCV bars from Binance.
pub fn load_reference_ohlcvs() -> Vec<RefBar> {
    load_records(OHLCV_PATH, "invalid OHLCV record")
}

/// Load single-value reference data (SMA, EMA).
pub fn load_ref_values(path: &str) -> Vec<RefValue> {
    load_records(path, "invalid reference record")
}

/// Load BB reference data (upper, middle, lower).
pub fn load_bb_ref(path: &str) -> Vec<RefBbValue> {
    load_records(path, "invalid BB reference record")
}

/// Assert two f64 values are within tolerance.
pub fn assert_near(actual: f64, expected: f64, tolerance: f64, context: &str) {
    let diff = (actual - expected).abs();
    assert!(
        diff <= tolerance,
        "{context}: expected {expected:.10}, got {actual:.10}, diff {diff:.2e} > tolerance {tolerance:.2e}"
    );
}

/// Creates perturbed versions of a bar to simulate live repaints.
///
/// Returns 2 intermediate bars (with shifted close/high/low) followed
/// by the original bar. All share the same `open_time`.
pub fn repaint_sequence(bar: &RefBar) -> Vec<RefBar> {
    let t = bar.open_time;
    vec![
        // First tick: only open is known, close near open
        RefBar {
            open: bar.open,
            high: bar.open * 1.001,
            low: bar.open * 0.999,
            close: bar.open * 1.0005,
            volume: bar.volume - 2.0,
            open_time: t,
        },
        // Mid-bar: partial movement toward final values
        RefBar {
            open: bar.open,
            high: bar.open.midpoint(bar.high),
            low: bar.open.midpoint(bar.low),
            close: bar.open.midpoint(bar.close),
            volume: bar.volume - 1.0,
            open_time: t,
        },
        // Final: real OHLCV values
        RefBar {
            open: bar.open,
            high: bar.high,
            low: bar.low,
            close: bar.close,
            volume: bar.volume,
            open_time: t,
        },
    ]
}

pub fn assert_values_match(
    bar_idx: usize,
    closed: Option<f64>,
    repainted: Option<f64>,
    tolerance: f64,
) {
    match (closed, repainted) {
        (None, None) => {} // both pre-convergence, fine
        (Some(c), Some(r)) => {
            let diff = (c - r).abs();
            assert!(
                diff <= tolerance,
                "diverged at bar {bar_idx}: closed={c:.10}, repainted={r:.10}, diff={diff:.2e}"
            );
        }
        (c, r) => {
            panic!("convergence mismatch at bar {bar_idx}: closed={c:?}, repainted={r:?}");
        }
    }
}

/// Assert BB values match between closed and repainted indicators.
pub fn assert_bb_values_match(
    bar_idx: usize,
    closed: Option<quantedge_ta::BbValue>,
    repainted: Option<quantedge_ta::BbValue>,
    tolerance: f64,
) {
    match (closed, repainted) {
        (None, None) => {}
        (Some(c), Some(r)) => {
            for (band, cv, rv) in [
                ("upper", c.upper(), r.upper()),
                ("middle", c.middle(), r.middle()),
                ("lower", c.lower(), r.lower()),
            ] {
                let diff = (cv - rv).abs();
                assert!(
                    diff <= tolerance,
                    "BB {band} diverged at bar {bar_idx}: closed={cv:.10}, repainted={rv:.10}, diff={diff:.2e}"
                );
            }
        }
        (c, r) => {
            panic!("BB convergence mismatch at bar {bar_idx}: closed={c:?}, repainted={r:?}");
        }
    }
}

/// Generate reference match + repaint tests for a single-value indicator.
///
/// Usage: `reference_test!(sma_20, Sma, SmaConfig::close(nz(20)), "tests/fixtures/data/sma-20-close.csv", 1e-6);`
#[allow(unused_macros)]
macro_rules! reference_test {
    ($name:ident, $ind:ty, $config:expr, $ref_path:expr, $tolerance:expr) => {
        mod $name {
            use super::fixtures::*;
            use quantedge_ta::*;
            use std::num::NonZero;

            fn nz(n: usize) -> NonZero<usize> {
                NonZero::new(n).unwrap()
            }

            #[test]
            fn matches_reference() {
                let bars = load_reference_ohlcvs();
                let reference = load_ref_values($ref_path);
                let config = $config;
                let mut ind = <$ind>::new(config);

                let mut ref_idx = 0;
                for bar in &bars {
                    ind.compute(bar);

                    if ref_idx < reference.len()
                        && bar.open_time == reference[ref_idx].open_time
                    {
                        let value = ind.value().unwrap_or_else(|| {
                            panic!("{} returned None at t={}", stringify!($name), bar.open_time)
                        });
                        assert_near(
                            value,
                            reference[ref_idx].expected,
                            $tolerance,
                            &format!(
                                "{} at bar {ref_idx} (t={})",
                                stringify!($name),
                                bar.open_time
                            ),
                        );
                        ref_idx += 1;
                    }
                }

                assert_eq!(
                    ref_idx,
                    reference.len(),
                    "not all reference values checked: {ref_idx}/{}",
                    reference.len()
                );
            }

            #[test]
            fn repaint_matches_closed() {
                let bars = load_reference_ohlcvs();
                let config = $config;
                let mut closed = <$ind>::new(config);
                let mut repainted = <$ind>::new(config);

                for (i, bar) in bars.iter().enumerate() {
                    closed.compute(bar);
                    for tick in repaint_sequence(bar) {
                        repainted.compute(&tick);
                    }
                    assert_values_match(i, closed.value(), repainted.value(), $tolerance);
                }
            }
        }
    };
}

#[allow(unused_imports)]
pub(crate) use reference_test;

fn load_records<D>(path: &str, expect_msg: &str) -> Vec<D>
where
    D: DeserializeOwned,
{
    let mut rdr =
        csv::Reader::from_path(path).unwrap_or_else(|e| panic!("failed to open {path}: {e}"));

    rdr.deserialize().map(|r| r.expect(expect_msg)).collect()
}
