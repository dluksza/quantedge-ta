//! Streaming technical analysis indicators for Rust.
//!
//! Indicators accept any type implementing [`Ohlcv`] and return
//! typed results via the `Indicator` trait. Values are `None`
//! until enough data has been received for convergence.

mod ohlcv;

pub use crate::ohlcv::{Ohlcv, Price, Timestamp};
