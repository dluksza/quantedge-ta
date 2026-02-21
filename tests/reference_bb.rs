mod fixtures;

use fixtures::{assert_near, load_bb_ref};
use quantedge_ta::{Bb, BbConfig};
use std::num::NonZero;

use crate::fixtures::{load_reference_ohlcvs, repaint_sequence};

const REF_PATH: &str = "tests/fixtures/data/bb-20-2-close.csv";

/// Tolerance: 1e-6 (~$0.000001 for BTC prices).
/// BB involves sqrt which adds minor FP noise beyond the SMA middle.
/// 1e-6 is tight enough to catch algorithmic bugs while allowing
/// representation differences.
const TOLERANCE: f64 = 1e-6;

#[test]
fn bb_20_2_close_matches_reference() {
    let bars = load_reference_ohlcvs();
    let reference = load_bb_ref(REF_PATH);

    let config = BbConfig::close(NonZero::new(20).unwrap());
    let mut bb = Bb::new(config);

    let mut ref_idx = 0;
    for bar in &bars {
        bb.compute(bar);

        if ref_idx < reference.len() && bar.open_time == reference[ref_idx].open_time {
            let value = bb
                .value()
                .unwrap_or_else(|| panic!("BB returned None at t={}", bar.open_time));
            let ctx = format!("BB(20,2) at bar {ref_idx} (t={})", bar.open_time);

            assert_near(
                value.upper(),
                reference[ref_idx].upper,
                TOLERANCE,
                &format!("{ctx} upper"),
            );
            assert_near(
                value.middle(),
                reference[ref_idx].middle,
                TOLERANCE,
                &format!("{ctx} middle"),
            );
            assert_near(
                value.lower(),
                reference[ref_idx].lower,
                TOLERANCE,
                &format!("{ctx} lower"),
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
fn bb_20_repaint_matches_closed() {
    let bars = load_reference_ohlcvs();

    let config = BbConfig::close(NonZero::new(20).unwrap());
    let mut closed = Bb::new(config);
    let mut repainted = Bb::new(config);

    for (i, bar) in bars.iter().enumerate() {
        closed.compute(bar);

        for tick in repaint_sequence(bar) {
            repainted.compute(&tick);
        }

        match (closed.value(), repainted.value()) {
            (None, None) => {}
            (Some(c), Some(r)) => {
                let ctx = format!("BB(20,2) at bar {i}");
                let pairs = [
                    ("upper", c.upper(), r.upper()),
                    ("middle", c.middle(), r.middle()),
                    ("lower", c.lower(), r.lower()),
                ];
                for (band, cv, rv) in pairs {
                    let diff = (cv - rv).abs();
                    assert!(
                        diff <= TOLERANCE,
                        "{ctx} {band}: closed={cv:.10}, repainted={rv:.10}, diff={diff:.2e}"
                    );
                }
            }
            (c, r) => {
                panic!("BB(20,2) convergence mismatch at bar {i}: closed={c:?}, repainted={r:?}");
            }
        }
    }
}
