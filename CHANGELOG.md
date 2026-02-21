# Changelog

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
