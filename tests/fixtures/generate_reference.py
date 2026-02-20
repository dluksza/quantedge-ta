#!/usr/bin/env python3
"""Generate reference indicator values from Binance OHLCV CSV.

Uses pure Python (no dependencies). Computes SMA, EMA, and BB
using the same algorithms as TradingView / TA-Lib:
- SMA: arithmetic mean of last N closes
- EMA: SMA seed, then alpha = 2/(N+1)
- BB: SMA middle, population std dev (÷N), 2 sigma bands

Usage:
    1. Download raw data:
       curl -sL "https://data.binance.vision/data/spot/monthly/klines/BTCUSDT/1h/BTCUSDT-1h-2025-01.zip" -o /tmp/btcusdt.zip
       unzip /tmp/btcusdt.zip -d /tmp
    2. Run: python3 tests/fixtures/generate_reference.py
    3. Verify: cargo test
"""

import csv
import math
import os
import sys

PERIOD = 20
OUTPUT_DIR = "tests/fixtures/data"


def read_binance_csv(path):
    """Read raw Binance kline CSV (no header, 12 columns)."""
    rows = []
    with open(path) as f:
        for row in csv.reader(f):
            rows.append(
                {
                    "open_time": int(row[0]),
                    "open": float(row[1]),
                    "high": float(row[2]),
                    "low": float(row[3]),
                    "close": float(row[4]),
                    "volume": float(row[5]),
                }
            )
    return rows


def compute_sma(closes, period):
    results = []
    for i in range(len(closes)):
        if i < period - 1:
            results.append(None)
        else:
            window = closes[i - period + 1 : i + 1]
            results.append(sum(window) / period)
    return results


def compute_ema(closes, period):
    results = [None] * (period - 1)
    alpha = 2.0 / (period + 1)
    seed = sum(closes[:period]) / period
    results.append(seed)
    prev = seed
    for i in range(period, len(closes)):
        val = alpha * closes[i] + (1 - alpha) * prev
        results.append(val)
        prev = val
    return results


def compute_bb(closes, period, mult=2.0):
    results = []
    for i in range(len(closes)):
        if i < period - 1:
            results.append(None)
        else:
            window = closes[i - period + 1 : i + 1]
            mean = sum(window) / period
            variance = sum((x - mean) ** 2 for x in window) / period
            std = math.sqrt(variance)
            results.append(
                {
                    "upper": mean + mult * std,
                    "middle": mean,
                    "lower": mean - mult * std,
                }
            )
    return results


def main():
    os.makedirs(OUTPUT_DIR, exist_ok=True)
    rows = read_binance_csv(sys.argv[1])
    closes = [r["close"] for r in rows]
    times = [r["open_time"] for r in rows]

    # OHLCV with header
    with open(f"{OUTPUT_DIR}/btcusdt-1h.csv", "w", newline="") as f:
        w = csv.writer(f)
        w.writerow(["open_time", "open", "high", "low", "close", "volume"])
        for r in rows:
            w.writerow(
                [
                    r["open_time"],
                    r["open"],
                    r["high"],
                    r["low"],
                    r["close"],
                    r["volume"],
                ]
            )

    # SMA
    sma_vals = compute_sma(closes, PERIOD)
    with open(f"{OUTPUT_DIR}/sma-20-close.csv", "w", newline="") as f:
        w = csv.writer(f)
        w.writerow(["open_time", "expected"])
        for i, val in enumerate(sma_vals):
            if val is not None:
                w.writerow([times[i], f"{val:.10f}"])

    # EMA
    ema_vals = compute_ema(closes, PERIOD)
    with open(f"{OUTPUT_DIR}/ema-20-close.csv", "w", newline="") as f:
        w = csv.writer(f)
        w.writerow(["open_time", "expected"])
        for i, val in enumerate(ema_vals):
            if val is not None:
                w.writerow([times[i], f"{val:.10f}"])

    # BB
    bb_vals = compute_bb(closes, PERIOD)
    with open(f"{OUTPUT_DIR}/bb-20-2-close.csv", "w", newline="") as f:
        w = csv.writer(f)
        w.writerow(["open_time", "upper", "middle", "lower"])
        for i, val in enumerate(bb_vals):
            if val is not None:
                w.writerow(
                    [
                        times[i],
                        f"{val['upper']:.10f}",
                        f"{val['middle']:.10f}",
                        f"{val['lower']:.10f}",
                    ]
                )

    print(
        f"Generated {sum(1 for v in sma_vals if v is not None)} SMA, "
        f"{sum(1 for v in ema_vals if v is not None)} EMA, "
        f"{sum(1 for v in bb_vals if v is not None)} BB reference values "
        f"from {len(rows)} OHLCV bars."
    )


if __name__ == "__main__":
    main()
