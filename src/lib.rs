//! Streaming technical analysis indicators for Rust.
//!
//! Indicators accept any type implementing [`Ohlcv`] and return
//! typed results via the `Indicator` trait. Values are `None`
//! until enough data has been received for convergence.

mod ema;
mod indicator;
mod ohlcv;
mod price_source;
mod price_window;
mod sma;

pub use crate::indicator::{Indicator, IndicatorConfig, IndicatorConfigBuilder};
pub use crate::ohlcv::{Ohlcv, Price, Timestamp};
pub use crate::price_source::PriceSource;

pub use crate::ema::{Ema, EmaConfig, EmaConfigBuilder};
pub use crate::sma::{Sma, SmaConfig, SmaConfigBuilder};

#[cfg(test)]
mod test_util;
