//! Streaming technical analysis indicators for Rust.
//!
//! Indicators accept any type implementing [`Ohlcv`] and return
//! typed results via the `Indicator` trait. Values are `None`
//! until enough data has been received for convergence.

mod indicator;
mod ohlcv;

pub use crate::indicator::{Indicator, IndicatorConfig, IndicatorConfigBuilder, PriceSource};
pub use crate::ohlcv::{Ohlcv, Price, Timestamp};
