# Test Fixtures

Reference data for validating indicator output against known-good values.

## OHLCV Data

**Source:** [Binance Public Data](https://data.binance.vision/)
**Symbol:** BTCUSDT, 1h candles, January 2025
**Rows:** 744 (full month)
**Timestamps:** Microseconds (Unix epoch)

Downloaded with:

```bash
curl -sL "https://data.binance.vision/data/spot/monthly/klines/BTCUSDT/1h/BTCUSDT-1h-2025-01.zip" \
  -o /tmp/btcusdt.zip
unzip /tmp/btcusdt.zip -d /tmp
```

The raw Binance CSV has no header and extra columns. `btcusdt-1h.csv` is the
trimmed version with headers: `open_time,open,high,low,close,volume`.

## Reference Values

Generated with the Python script below using **stdlib only** (no TA-Lib,
no pandas). The algorithms match TradingView and TA-Lib:

- **SMA:** Arithmetic mean of the last N closing prices
- **EMA:** SMA seed for the first N bars, then `alpha * price + (1 - alpha) * prev`
  where `alpha = 2 / (N + 1)`
- **BB:** SMA middle band, population standard deviation (divide by N, not N-1),
  upper/lower = middle +/- 2 * sigma

All reference files use period 20 on closing price.

### Generator Script

See [generate_reference.py](./generate_reference.py)

### Regenerating

1. Download the raw Binance CSV (see curl command above)
2. Run the script: `python3 tests/fixtures/generate_reference.py /tmp/BTCUSDT-1h-2025-01.csv`
3. Verify: `cargo test`
