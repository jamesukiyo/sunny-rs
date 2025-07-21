#![forbid(unsafe_code)]
#![warn(clippy::pedantic)]
#![allow(clippy::cast_possible_truncation)]
#![allow(dead_code)]

mod args;
mod printer;

use args::Args;
use printer::printer;

use clap::Parser;
use serde::Deserialize;
use std::ops::Sub;

#[derive(Deserialize, Debug)]
struct WeatherResponse {
	name: String,
	main: Main,
	sys: Sys,
	weather: Vec<Weather>,
}

#[derive(Deserialize, Debug)]
struct Weather {
	main: String,
	description: String,
	icon: String,
}

#[derive(Deserialize, Debug)]
struct Main {
	temp: f64,
	feels_like: f64,
	humidity: i32,
}

#[derive(Deserialize, Debug)]
struct Sys {
	country: String,
}

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

fn main() -> Result<(), Box<dyn std::error::Error>> {
	let args = Args::parse();

	let api_key = if args.key.is_empty() {
		std::env::var("OPEN_WEATHER_API_KEY")
			.expect("OPEN_WEATHER_API_KEY not set")
	} else {
		args.key.trim().to_string()
	};

	let city = args.city.trim().to_lowercase();

	let url = format!(
		"https://api.openweathermap.org/data/2.5/weather?q={city}&appid={api_key}"
	);

	let res = reqwest::blocking::get(&url)?.json::<WeatherResponse>()?;

	let output = Output {
		country: res.sys.country,
		city: capitalize(&res.name),
		temp: res.main.temp.sub(273.15).round() as i64,
		feels_like: res.main.feels_like.sub(273.15).round() as i64,
		humidity: res.main.humidity.to_string(),
		type_of: res.weather[0].main.clone(),
		description: res.weather[0].description.clone(),
		icon: res.weather[0].icon.clone(),
	};

	printer(args.raw, args.simple, args.fahrenheit, &output);
	Ok(())
}

fn capitalize(s: &str) -> String {
	s.chars()
		.enumerate()
		.map(|(i, c)| {
			if i == 0 || s.chars().nth(i - 1) == Some(' ') {
				c.to_uppercase().collect::<String>()
			} else {
				c.to_lowercase().collect::<String>()
			}
		})
		.collect()
}
