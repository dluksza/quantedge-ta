# Changelog

## [Unreleased]

### Changed

- `Sma`, `Ema`, and `Bb` now expose `new()`, `compute()`, and `value()` as inherent methods, `use quantedge_ta::Indicator` is no longer required for basic usage.
- Config and builder types (`SmaConfig`, `EmaConfig`, `BbConfig` and their builders) now expose trait methods as inherent methods. `use quantedge_ta::IndicatorConfig` and `use quantedge_ta::IndicatorConfigBuilder` are no longer required for basic usage.
- Benchmark harness: deterministic codegen (`codegen-units = 1`, `lto = "thin"`), lower-overhead batching (`SmallInput`), and tuned tick-group sampling (200 samples, 5s warmup, 10s measurement, 3% noise threshold).
- `PriceWindow` now uses a `const SUM_OF_SQUARES: bool` generic so SMA no longer computes unused sum-of-squares on every tick.
- Replaced `VecDeque` with a custom `RingBuffer` in `PriceWindow`, halves buffer memory (`f64` vs `Option<f64>`), eliminates redundant modulo operations, and improves inlining. SMA and BB stream throughput improved 24-33%, tick latency improved 8-31%.

## [0.1.0] - 2025-02-20

Initial release.

### Added

- SMA (Simple Moving Average)
- EMA (Exponential Moving Average) with configurable convergence enforcement
- Bollinger Bands (upper, middle, lower)
- `Ohlcv` trait for zero-copy integration with user types
- 9 price sources (Close, Open, High, Low, HL2, HLC3, OHLC4, HLCC4, TrueRange)
- Live bar repainting via `open_time` comparison
- Reference tests against 744 BTC/USDT bars
- Criterion benchmarks (stream + tick)

[0.1.0]: https://github.com/dluksza/quantedge-ta/releases/tag/v0.1.0
