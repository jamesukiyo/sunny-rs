#![forbid(unsafe_code)]
#![warn(clippy::pedantic)]
#![allow(clippy::cast_possible_truncation)]
#![allow(dead_code)]

mod args;

use args::Args;
use clap::Parser;
use serde::Deserialize;
use std::ops::Sub;

#[derive(Deserialize, Debug)]
struct WeatherResponse {
	name: String,
	main: Main,
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

#[derive(Debug)]
struct Output {
	temp_c: i64,
	feels_like_c: i64,
	humidity: i32,
	type_of: String,
	description: String,
	icon: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	let args = Args::parse();
	let api_key = std::env::var("OPEN_WEATHER_API_KEY")
		.expect("OPEN_WEATHER_API_KEY not set");

	let city = args.city.trim().to_lowercase();

	let url = format!(
		"https://api.openweathermap.org/data/2.5/weather?q={city}&appid={api_key}"
	);

	println!("url: {url}");

	let res = reqwest::get(&url).await?.json::<WeatherResponse>().await?;

	if args.simple {
		println!("simple output");
	}

	let output = Output {
		temp_c: res.main.temp.sub(273.15).round() as i64,
		feels_like_c: res.main.feels_like.sub(273.15).round() as i64,
		humidity: res.main.humidity,
		type_of: res.weather[0].main.clone(),
		description: res.weather[0].description.clone(),
		icon: res.weather[0].icon.clone(),
	};

	println!("{output:#?}");
	Ok(())
}
