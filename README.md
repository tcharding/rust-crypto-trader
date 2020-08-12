Cryptocurrency trading in Rust
==============================

Algorithmic trading against the Independent Reserve cryptocurrency
exchange https://www.independentreserve.com/

## Usage

Different functionality is available on the various branches.

### Spread bot

Gets current spread for AUD/BTC every second, writes min/max values to
file every 5 minutes.

```
    git checkout spread-bot
    cargo run
```

Output file is hardcoded in `main.rs`.

## Testing

`master` has a test function call to [partially] test the exchange API.

```
market::test_ir_api(config.keys.clone()).await;
```

### API Keys

You will need to put your API keys somewhere, currently the path is
hardcoded in main, either change this or put your keys in:

```
const CONFIG_FILE: &str = ".config/crypto-trader/config.toml";
```

Sample config file:

```
[keys]

    [keys.read]
    key = "xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx"
    secret = "xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx"
```

## Contributing

Contributions and ideas welcome, use at your own discretion.
