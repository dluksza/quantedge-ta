mod fixtures;

use fixtures::{assert_near, load_ref_values};
use quantedge_ta::{Ema, EmaConfig, Indicator};
use std::num::NonZero;

use crate::fixtures::{assert_values_match, load_reference_ohlcvs, repaint_sequence};

const REF_PATH: &str = "tests/fixtures/data/ema-20-close.csv";

/// Tolerance: 1e-6 (~$0.000001 for BTC prices).
/// EMA accumulates floating-point error over time but the reference
/// is computed identically (SMA seed + alpha smoothing). Any drift
/// is from f64 representation, not algorithmic divergence.
const TOLERANCE: f64 = 1e-6;

#[test]
fn ema_20_close_matches_reference() {
    let bars = load_reference_ohlcvs();
    let reference = load_ref_values(REF_PATH);

    let config = EmaConfig::close(NonZero::new(20).unwrap());
    let mut ema = Ema::new(config);

    let mut ref_idx = 0;
    for bar in &bars {
        ema.compute(bar);

        if ref_idx < reference.len() && bar.open_time == reference[ref_idx].open_time {
            let value = ema
                .value()
                .unwrap_or_else(|| panic!("EMA returned None at t={}", bar.open_time));
            assert_near(
                value,
                reference[ref_idx].expected,
                TOLERANCE,
                &format!("EMA(20) at bar {ref_idx} (t={})", bar.open_time),
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
fn ema_20_repaint_matches_closed() {
    let bars = load_reference_ohlcvs();

    let config = EmaConfig::close(NonZero::new(20).unwrap());
    let mut closed = Ema::new(config);
    let mut repainted = Ema::new(config);

    for (i, bar) in bars.iter().enumerate() {
        closed.compute(bar);

        for tick in repaint_sequence(bar) {
            repainted.compute(&tick);
        }

        assert_values_match(i, closed.value(), repainted.value(), TOLERANCE);
    }
}
