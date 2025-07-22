#![forbid(unsafe_code)]
#![warn(clippy::pedantic)]
#![allow(clippy::cast_possible_truncation)]
#![allow(dead_code)]

mod args;
mod config;
mod fetch;
mod printer;

use args::Args;
use fetch::fetch_weather;
use printer::printer;

use clap::Parser;
use color_eyre::eyre::{Result, eyre};
use std::ops::Sub;

#[derive(Debug)]
struct Output {
	country: String,
	city: String,
	temp: i64,
	feels_like: i64,
	humidity: String,
	type_of: String,
	description: String,
	icon: String,
}

enum HeaderFooter {
	Clean,
	Header,
	Footer,
	Both,
}

enum OutputStyle {
	Pretty,
	Simple,
	Raw,
}

struct PrintOpts {
	use_fahrenheit: bool,
	header_footer: HeaderFooter,
	output_style: OutputStyle,
	use_icons: bool,
}

fn main() -> Result<()> {
	color_eyre::install()?;
	let args = Args::parse();
	let config = config::Config::load()?;

	// api key: args > config > env
	let api_key = if !args.key.is_empty() {
		args.key.trim()
	} else if !config.api_key.is_empty() {
		config.api_key.trim()
	} else {
		&std::env::var("OPEN_WEATHER_API_KEY")
			.map_err(|_| eyre!("No API key specified!\nUse -k, OPEN_WEATHER_API_KEY in environment or set 'api_key' in ~/.config/sunny.toml"))?
	};

	// city: args > config
	let city = if !args.city.is_empty() {
		args.city.trim()
	} else if !config.city.is_empty() {
		config.city.trim()
	} else {
		return Err(eyre!(
			"No city specified, use 'sunny <city>' or set 'city' in ~/.config/sunny.toml"
		));
	};

	// fahrenheit: args > config
	let use_fahrenheit = args.fahrenheit || config.use_fahrenheit;

	// header/footer display
	let header_footer =
		if args.clean_output || (args.no_header && args.no_footer) {
			HeaderFooter::Clean
		} else {
			let show_header = !args.no_header && config.show_header;
			let show_footer = !args.no_footer && config.show_footer;

			match (show_header, show_footer) {
				(true, true) => HeaderFooter::Both,
				(true, false) => HeaderFooter::Header,
				(false, true) => HeaderFooter::Footer,
				(false, false) => HeaderFooter::Clean,
			}
		};

	// output style
	let output_style = if args.raw {
		OutputStyle::Raw
	} else if args.simple {
		OutputStyle::Simple
	} else {
		OutputStyle::Pretty
	};

	let use_icons = args.icons || config.icons;

	let print_opts = PrintOpts {
		use_fahrenheit,
		header_footer,
		output_style,
		use_icons,
	};

	let data = fetch_weather(city, api_key)?;

	let weather = data
		.weather
		.into_iter()
		.next()
		.ok_or_else(|| eyre!("No weather data received"))?;

	let output = Output {
		country: data.sys.country,
		city: capitalise(&data.name),
		temp: data.main.temp.sub(273.15).round() as i64,
		feels_like: data.main.feels_like.sub(273.15).round() as i64,
		humidity: data.main.humidity.to_string(),
		type_of: weather.main,
		description: weather.description,
		icon: weather.icon,
	};

	printer(&print_opts, &output);
	Ok(())
}

fn capitalise(s: &str) -> String {
	let mut result = String::with_capacity(s.len());
	let mut capitalise_next = true;

	for ch in s.chars() {
		if capitalise_next {
			result.extend(ch.to_uppercase());
			capitalise_next = false;
		} else if ch == ' ' {
			result.push(ch);
			capitalise_next = true;
		} else {
			result.extend(ch.to_lowercase());
		}
	}
	result
}
