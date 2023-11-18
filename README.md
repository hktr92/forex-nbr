# Forex of National Bank of Romania

This is the official Rust library for the [National Bank of Romania](https://www.bnr.ro/)'s [Foreign Exchange Reference Rates](https://www.bnr.ro/Cursurile-pietei-valutare-in-format-XML-3424.aspx) API.


## Usage

```rust
let api = forex_nbr::NbrApi::default();
///
// to fetch the latest available rates
let latest = api.latest().await?;
///
// to fetch the last 10 days rates
let last_10_days = api.last_10_days().await?;
///
// to fetch historical rates for a given year
let historical = api.historical(2019).await?;
///
```