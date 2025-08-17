# system-analyzer

## Setup development environment

Use `DATABASE_URL="sqlite://database.sqlite?mode=rwc" cargo sqlx prepare` for preparing sqlx cache.

You need to have sqlx-cli installed.

## Benchmarks

Walk through a minimal cdebootstrap installed debian bookworm

```
quantenregen@quantenregen-home-i37100:~/Schreibtisch/system-analyzer$ time ./target/release/system-analyzer

real    1m22,840s
user    0m3,833s
sys     0m4,088s
```
