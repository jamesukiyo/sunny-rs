<h1 align="center">⛅ sunny-rs ⛅</h1>

<p align="center">A simple weather app for your terminal, written in Rust.</p>

<div align="center">
    <img src="/showcase/showcase.png" width="400px">
</div>

## Features
- check the weather for any city today or tomorrow
- fast (~130ms with fetch, sunny itself only takes 0.5 to 3ms depending on output style)
- 3 output styles: fancy, simple, raw
- fahrenheit support
- emojis or nerd font icons if supported
- toml config file

## Usage

### Install
With cargo:
```sh
cargo install sunny-cli
```

With npm/bun/pnpm:
```sh
npm install -g @jamesukiyo/sunny-cli
bun install -g @jamesukiyo/sunny-cli
pnpm install -g @jamesukiyo/sunny-cli
```

You will need a free API key from [OpenWeatherMap](https://openweathermap.org/api).

A nerd font and terminal that supports emojis is recommended (see below).

See the [#run](#run) section for info on how to use sunny.

### Config

A config file is created on first run at `$HOME/.config/sunny.toml`.

Inside it you can set the following:
- `city`: the city to get the weather for [default: ""]
- `api_key`: your API key for OpenWeatherMap [default: ""]
- `use_fahrenheit`: whether to use fahrenheit for temperature [default: false]
- `show_footer`: whether to show the footer [default: true]
- `show_header`: whether to show the header [default: true]
- `icons`: whether to enable nerd font and emoji icons [default: true]

>[!INFO]
> If icons are enabled, an emoji is tried first if supported and then a nerd
> font icon. If neither work, the layout may be incorrect so try disabling icons
> or trying a nerd font.

See `example_config.toml` for an example.

>[!HINT]
> CLI arguments will override corresponding config values.

I recommend using the config file. Then you can run `sunny` without any
arguments and see the weather quickly for today.

It's also easier to override the city this way but still have a good default.
```sh
sunny        # uses your default city
sunny paris  # allows override
```


### Run

For cargo installations:
```sh
sunny                   # today (city in config)
sunny t                 # tomorrow (city in config)
sunny paris             # today in paris
sunny tomorrow paris    # tomorrow in paris
sunny paris --simple    # today in paris, simple output
sunny t paris --simple  # tomorrow in paris, simple output
```

For npm/bun/pnpm installations prefix with your package managers executor:
```sh
npx sunny
bunx sunny
pnpx sunny
```


Full help:
```
Usage: sunny [OPTIONS] [CITY] [COMMAND]

Commands:
  today     Get today's weather (default)
  tomorrow  Get tomorrow's weather
  t         Short alias for tomorrow
  help      Print this message or the help of the given subcommand(s)

Arguments:
  [CITY]  City to get the weather for [default: ]

Options:
  -s, --simple      Simpler output (no styling)
  -r, --raw         Raw JSON output
  -k, --key <KEY>   API key for OpenWeatherMap [default: ]
  -f, --fahrenheit  Use fahrenheit for temperature
  -F, --no-footer   Hide the credits footer from output
  -H, --no-header   Hide the header from output
  -c, --clean       Alias for --no-header --no-footer
  -i, --no-icons    Disable icons - good for non-nerd fonts or lack of emoji support
  -h, --help        Print help
```

## License

Copyright (c) James Plummer <jamesp2001@live.co.uk>

This project is licensed under the MIT license ([LICENSE] or <http://opensource.org/licenses/MIT>)

[LICENSE]: ./LICENSE
