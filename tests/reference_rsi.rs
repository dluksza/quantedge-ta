mod fixtures;

use fixtures::{assert_near, load_ref_values};
use quantedge_ta::{Rsi, RsiConfig};
use std::num::NonZero;

use crate::fixtures::{assert_values_match, load_reference_ohlcvs, repaint_sequence};

const REF_PATH: &str = "tests/fixtures/data/rsi-14-close.csv";

/// Tolerance: 1e-6.
/// RSI uses Wilder's smoothing — infinite memory but deterministic given same seed.
const TOLERANCE: f64 = 1e-6;

#[test]
fn rsi_14_close_matches_reference() {
    let bars = load_reference_ohlcvs();
    let reference = load_ref_values(REF_PATH);

    let config = RsiConfig::close(NonZero::new(14).unwrap());
    let mut rsi = Rsi::new(config);

    let mut ref_idx = 0;
    for bar in &bars {
        rsi.compute(bar);

        if ref_idx < reference.len() && bar.open_time == reference[ref_idx].open_time {
            let value = rsi
                .value()
                .unwrap_or_else(|| panic!("RSI returned None at t={}", bar.open_time));
            assert_near(
                value,
                reference[ref_idx].expected,
                TOLERANCE,
                &format!("RSI(14) at bar {ref_idx} (t={})", bar.open_time),
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
fn rsi_14_repaint_matches_closed() {
    let bars = load_reference_ohlcvs();

    let config = RsiConfig::close(NonZero::new(14).unwrap());
    let mut closed = Rsi::new(config);
    let mut repainted = Rsi::new(config);

    for (i, bar) in bars.iter().enumerate() {
        closed.compute(bar);

        for tick in repaint_sequence(bar) {
            repainted.compute(&tick);
        }

        assert_values_match(i, closed.value(), repainted.value(), TOLERANCE);
    }
}
