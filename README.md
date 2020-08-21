Cryptocurrency trading in Rust
==============================

Algorithmic trading against the Independent Reserve cryptocurrency
exchange https://www.independentreserve.com/

## Usage

See `crypto-trader --help` for usage.

### API Keys

You will need to put your API keys somewhere, currently the path is
hardcoded in main, either change this or put your keys in:

`~/.config/crypto-trader/config.toml`

Run `crypto-trader --dump-config` to see the contents of the configuration file being read.

Sample config file:

```
[ir.read_only]
api_key = "xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx"
api_secret = "xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx"
```

## Testing

Run `crypto-trader test` to test the exchange API.

## Spread bot

Long running process to scrape orderbook data from the exchange.

`screen -dmSL bot crypto-trader spread-trader`

Output file is hardcoded in `main.rs`, currently `./spread-bot.log`.

## Contributing

Contributions and ideas welcome, use at your own discretion.
