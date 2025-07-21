<h1 align="center">⛅ sunny-rs ⛅</h1>

<p align="center">A simple CLI weather app written in Rust.</p>

<div align="center">
    <img src="/showcase/showcase.png" width="400px">
</div>

## Features
- check the current weather for any city
- fast (120ms with fetch, sunny itself only takes 0.5 to 3ms depending on output style)
- 3 output styles: fancy, simple, raw
- fahrenheit support
- toml config file

## Usage

### Install
```sh
cargo install sunny-cli
```

You will need an API key from [OpenWeatherMap](https://openweathermap.org/api).

### Config

A config file is created on first run at `$HOME/.config/sunny.toml`.

Inside it you can set the following:
- `city`: the city to get the weather for
- `api_key`: your API key for OpenWeatherMap
- `use_fahrenheit`: whether to use fahrenheit for temperature

See `example_config.toml` for an example.

>[!NOTE]
> CLI arguments will override config values.

I recommend using the config file. Then you can run `sunny` without any
arguments and see the weather quickly.

It's also easier to override the city this way but still have a good default.
```sh
sunny        # uses your default city
sunny paris  # allows override
```


### Run
```
Usage: sunny [OPTIONS] [CITY]

Arguments:
  [CITY]  City to get the weather for (overrides config) [default: ]

Options:
  -s, --simple      Simpler output (no colours)
      --raw         Raw JSON output
  -k, --key <KEY>   API key for OpenWeatherMap (overrides config) [default: ]
  -f, --fahrenheit  Use fahrenheit for temperature
  -h, --help        Print help
```

## License

Copyright (c) James Plummer <jamesp2001@live.co.uk>

This project is licensed under the MIT license ([LICENSE] or <http://opensource.org/licenses/MIT>)

[LICENSE]: ./LICENSE
