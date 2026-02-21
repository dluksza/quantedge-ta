mod fixtures;

use fixtures::{assert_near, load_ref_values};
use quantedge_ta::{Sma, SmaConfig};
use std::num::NonZero;

use crate::fixtures::{assert_values_match, load_reference_ohlcvs, repaint_sequence};

const REF_PATH: &str = "tests/fixtures/data/sma-20-close.csv";

/// Tolerance: 1e-6 (~$0.000001 for BTC prices).
/// SMA is pure arithmetic over a fixed window — no accumulated drift.
const TOLERANCE: f64 = 1e-6;

#[test]
fn sma_20_close_matches_reference() {
    let bars = load_reference_ohlcvs();
    let reference = load_ref_values(REF_PATH);

    let config = SmaConfig::close(NonZero::new(20).unwrap());
    let mut sma = Sma::new(config);

    let mut ref_idx = 0;
    for bar in &bars {
        sma.compute(bar);

        if ref_idx < reference.len() && bar.open_time == reference[ref_idx].open_time {
            let value = sma
                .value()
                .unwrap_or_else(|| panic!("SMA returned None at t={}", bar.open_time));
            assert_near(
                value,
                reference[ref_idx].expected,
                TOLERANCE,
                &format!("SMA(20) at bar {ref_idx} (t={})", bar.open_time),
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
fn sma_20_repaint_matches_closed() {
    let bars = load_reference_ohlcvs();

    let config = SmaConfig::close(NonZero::new(20).unwrap());
    let mut closed = Sma::new(config);
    let mut repainted = Sma::new(config);

    for (i, bar) in bars.iter().enumerate() {
        // Closed: single compute
        closed.compute(bar);

        // Repainted: intermediate ticks then final
        for tick in repaint_sequence(bar) {
            repainted.compute(&tick);
        }

        assert_values_match(i, closed.value(), repainted.value(), TOLERANCE);
    }
}
