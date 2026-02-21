//! Streaming technical analysis indicators for Rust.
//!
//! Indicators accept any type implementing [`Ohlcv`] and return
//! typed results. Values are `None` until enough data has been
//! received for convergence.
//!
//! Each indicator type ([`Sma`], [`Ema`], [`Bb`]) exposes
//! [`new`](Sma::new), [`compute`](Sma::compute), and
//! [`value`](Sma::value) as inherent methods — no trait import
//! needed. Import [`Indicator`] only for generic code.

mod bb;
mod ema;
mod indicator;
mod ohlcv;
mod price_source;
mod price_window;
mod ring_buffer;
mod sma;

pub use crate::indicator::{Indicator, IndicatorConfig, IndicatorConfigBuilder};
pub use crate::ohlcv::{Ohlcv, Price, Timestamp};
pub use crate::price_source::PriceSource;

pub use crate::bb::{Bb, BbConfig, BbConfigBuilder, BbValue, StdDev};
pub use crate::ema::{Ema, EmaConfig, EmaConfigBuilder};
pub use crate::sma::{Sma, SmaConfig, SmaConfigBuilder};

macro_rules! impl_indicator_methods {
    ($type:ty, $config:ty, $output:ty) => {
        impl $type {
            /// See [`Indicator::new`].
            #[must_use]
            pub fn new(config: $config) -> Self {
                <Self as Indicator>::new(config)
            }

            /// See [`Indicator::compute`].
            #[inline]
            pub fn compute(&mut self, kline: &impl Ohlcv) -> Option<$output> {
                <Self as Indicator>::compute(self, kline)
            }

            /// See [`Indicator::value`].
            #[must_use]
            #[inline]
            pub fn value(&self) -> Option<$output> {
                <Self as Indicator>::value(self)
            }
        }
    };
}

impl_indicator_methods!(Sma, SmaConfig, Price);
impl_indicator_methods!(Ema, EmaConfig, Price);
impl_indicator_methods!(Bb, BbConfig, BbValue);

#[cfg(test)]
mod test_util;

#[cfg(test)]
mod inherent_methods {
    use super::{Bb, BbConfig, BbValue, Ema, EmaConfig, Ohlcv, Price, Sma, SmaConfig, Timestamp};
    use std::num::NonZero;

    struct Bar(f64, u64);
    impl Ohlcv for Bar {
        fn open(&self) -> Price {
            self.0
        }
        fn high(&self) -> Price {
            self.0
        }
        fn low(&self) -> Price {
            self.0
        }
        fn close(&self) -> Price {
            self.0
        }
        fn open_time(&self) -> Timestamp {
            self.1
        }
    }

    #[test]
    fn sma_without_indicator_import() {
        let mut sma = Sma::new(SmaConfig::close(NonZero::new(2).unwrap()));
        assert_eq!(sma.compute(&Bar(10.0, 1)), None);
        assert_eq!(sma.compute(&Bar(20.0, 2)), Some(15.0));
        assert_eq!(sma.value(), Some(15.0));
    }

    #[test]
    fn ema_without_indicator_import() {
        let mut ema = Ema::new(EmaConfig::close(NonZero::new(2).unwrap()));
        assert_eq!(ema.compute(&Bar(10.0, 1)), None);
        assert!(ema.compute(&Bar(20.0, 2)).is_some());
        assert!(ema.value().is_some());
    }

    #[test]
    fn bb_without_indicator_import() {
        let mut bb = Bb::new(BbConfig::close(NonZero::new(2).unwrap()));
        assert!(bb.compute(&Bar(10.0, 1)).is_none());
        let v: Option<BbValue> = bb.compute(&Bar(20.0, 2));
        assert!(v.is_some());
        assert!(bb.value().is_some());
    }
}
