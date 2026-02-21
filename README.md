# quantedge-ta

[![CI](https://github.com/dluksza/quantedge-ta/actions/workflows/ci.yml/badge.svg)](https://github.com/dluksza/quantedge-ta/actions/workflows/ci.yml)
[![codecov](https://codecov.io/gh/dluksza/quantedge-ta/branch/main/graph/badge.svg)](https://codecov.io/gh/dluksza/quantedge-ta)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE-MIT)

A streaming technical analysis library for Rust. Correct, tested, documented.

## Features

### Type-safe convergence

Indicators return `Option<Self::Output>`. No value until there's enough data.
No silent NaN, no garbage early values. The type system enforces correctness.
For indicators with infinite memory (EMA), convergence enforcement is
configurable: opt in to suppress values until the seed's influence has decayed
below 1%.

### Bring your own data

Indicators accept any type implementing the `Ohlcv` trait. No forced conversion
to a library-specific struct. Implement five required methods on your existing
type and you're done. Volume has a default implementation for data sources that
don't provide it.

### O(1) incremental updates

Indicators maintain running state and update in constant time per tick. No
re-scanning the window.

### Live repainting

Indicators track bar boundaries using `open_time`. A kline with a new
`open_time` advances the window; same `open_time` replaces the current value.
Useful for trading terminals and real-time systems that need indicator values
on forming bars.

### Typed outputs

Each indicator defines its own output type via an associated type on the
`Indicator` trait. SMA and EMA return `f64`. Bollinger Bands returns
`BbValue { upper, middle, lower }`. No downcasting, no enums, full type
safety.

## Usage

```rust
use quantedge_ta::{Sma, SmaConfig};
use std::num::NonZero;

let mut sma = Sma::new(SmaConfig::close(NonZero::new(20).unwrap()));

for kline in stream {
    if let Some(value) = sma.compute(&kline) {
        println!("SMA(20): {value}");
    }
    // None = not enough data yet
}
```

Bollinger Bands returns a struct:

```rust
use quantedge_ta::{
    Bb, BbConfig, IndicatorConfig, IndicatorConfigBuilder,
};
use std::num::NonZero;

let config = BbConfig::builder()
    .length(NonZero::new(20).unwrap())
    .build();
let mut bb = Bb::new(config);

for kline in stream {
    if let Some(value) = bb.compute(&kline) {
        println!("BB upper: {}, middle: {}, lower: {}",
            value.upper(), value.middle(), value.lower());
    }
}
```

Custom standard deviation multiplier:

```rust
use quantedge_ta::{BbConfig, StdDev, IndicatorConfig, IndicatorConfigBuilder};
use std::num::NonZero;

let config = BbConfig::builder()
    .length(NonZero::new(20).unwrap())
    .std_dev(StdDev::new(1.5))
    .build();
```

Live data with repainting:

```rust
// Open kline arrives (open_time = 1000)
sma.compute(&open_kline);    // computes with current bar

// Same bar, new trade (open_time = 1000, updated close)
sma.compute(&updated_kline); // replaces current bar value

// Next bar (open_time = 2000)
sma.compute(&next_kline);    // advances the window
```

The caller controls bar boundaries. The library handles the rest.

### Indicator Trait

Each indicator defines its output type. No downcasting needed:

```rust
trait Indicator: Sized + Clone + Display + Debug {
    type Config: IndicatorConfig;
    type Output: Send + Sync + Display + Debug;

    fn new(config: Self::Config) -> Self;
    fn compute(&mut self, kline: &impl Ohlcv) -> Option<Self::Output>;
    fn value(&self) -> Option<Self::Output>;
}

// Sma: Output = f64
// Ema: Output = f64
// Bb:  Output = BbValue { upper: f64, middle: f64, lower: f64 }
```

### Ohlcv Trait

Implement the `Ohlcv` trait on your own data type:

```rust
use quantedge_ta::{Ohlcv, Price, Timestamp};

struct MyKline {
    open: f64,
    high: f64,
    low: f64,
    close: f64,
    open_time: u64,
}

impl Ohlcv for MyKline {
    fn open(&self) -> Price { self.open }
    fn high(&self) -> Price { self.high }
    fn low(&self) -> Price { self.low }
    fn close(&self) -> Price { self.close }
    fn open_time(&self) -> Timestamp { self.open_time }
    // fn volume(&self) -> f64 { 0.0 }  -- default, override if needed
}
```

### Convergence

SMA and BB converge as soon as the window fills (`length` bars). EMA has
infinite memory; the SMA seed influences all subsequent values. `EmaConfig`
provides methods to control this:

- `enforce_convergence()` -- when `true`, `compute()` returns `None` until
  the seed's contribution decays below 1%.
- `required_bars_to_converge()` -- returns the number of bars needed.

```rust
use quantedge_ta::{EmaConfig, IndicatorConfig, IndicatorConfigBuilder};
use std::num::NonZero;

let config = EmaConfig::builder()
    .length(NonZero::new(20).unwrap())
    .enforce_convergence(true) // None until ~63 bars
    .build();
config.required_bars_to_converge(); // 63 = 3 * (20 + 1)
```

Use `required_bars_to_converge()` to determine how much history to fetch before
going live.

### Price Sources

Each indicator is configured with a `PriceSource` that determines which value
to extract from the Ohlcv input:

| Source    | Formula                                                   |
|-----------|-----------------------------------------------------------|
| Close     | close                                                     |
| Open      | open                                                      |
| High      | high                                                      |
| Low       | low                                                       |
| HL2       | (high + low) / 2                                          |
| HLC3      | (high + low + close) / 3                                  |
| OHLC4     | (open + high + low + close) / 4                           |
| HLCC4     | (high + low + close + close) / 4                          |
| TrueRange | max(high - low, \|high - prev_close\|, \|low - prev_close\|) |

## Indicators

### v0.1

| Indicator | Output     | Description                         |
|-----------|------------|-------------------------------------|
| SMA       | `f64`      | Simple Moving Average               |
| EMA       | `f64`      | Exponential Moving Average          |
| BB        | `BbValue`  | Bollinger Bands (upper, mid, lower) |

### Planned

RSI, MACD, ATR, CHOP, and more.

## Benchmarks

Measured with [Criterion.rs](https://github.com/bheisler/criterion.rs) on 744
BTC/USDT 1-hour bars from Binance.

**Stream** measures end-to-end throughput including window fill.
**Tick** isolates steady-state per-bar cost on a fully converged indicator.

**Hardware:** Apple M3 Max (16 cores), 48 GB RAM, macOS 26.3, rustc 1.93.1,
`--release` profile.

### Stream — process 744 bars from cold start

| Indicator | Period | Time (median) | Throughput     |
|-----------|--------|---------------|----------------|
| SMA       | 20     | 3.29 µs       | 226 Melem/s    |
| SMA       | 200    | 2.95 µs       | 252 Melem/s    |
| EMA       | 20     | 3.29 µs       | 226 Melem/s    |
| EMA       | 200    | 3.32 µs       | 224 Melem/s    |
| BB        | 20     | 5.59 µs       | 133 Melem/s    |
| BB        | 200    | 4.36 µs       | 171 Melem/s    |

### Tick — single `compute()` on a converged indicator

| Indicator | Period | Time (median) |
|-----------|--------|---------------|
| SMA       | 20     | 12.8 ns       |
| SMA       | 200    | 17.7 ns       |
| EMA       | 20     | 9.71 ns       |
| EMA       | 200    | 8.61 ns       |
| BB        | 20     | 18.0 ns       |
| BB        | 200    | 20.6 ns       |

Run locally:

```bash
cargo bench              # all benchmarks
cargo bench -- stream    # stream only
cargo bench -- tick      # single-tick only
```

## Minimum Supported Rust Version

1.93

## Licence

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT License ([LICENSE-MIT](LICENSE-MIT) or
  <http://opensource.org/licenses/MIT>)

at your option.

## Contributing

Contributions welcome. Please open an issue before submitting large changes.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 licence, shall
be dual-licensed as above, without any additional terms or conditions.
