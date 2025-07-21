use color_eyre::eyre::{Result, eyre};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct WeatherResponse {
	pub name: String,
	pub main: Main,
	pub sys: Sys,
	pub weather: Vec<Weather>,
}

#[derive(Deserialize, Debug)]
pub struct Weather {
	pub main: String,
	pub description: String,
	pub icon: String,
}

#[derive(Deserialize, Debug)]
pub struct Main {
	pub temp: f64,
	pub feels_like: f64,
	pub humidity: i32,
}

#[derive(Deserialize, Debug)]
pub struct Sys {
	pub country: String,
}

pub fn fetch_weather(city: &str, api_key: &str) -> Result<WeatherResponse> {
	let url = format!(
		"https://api.openweathermap.org/data/2.5/weather?q={city}&appid={api_key}"
	);

	let res = reqwest::blocking::get(&url)?;

	let status = res.status();
	if !status.is_success() {
		let error_message = match status.as_u16() {
			400 => "Bad request - check the provided city name",
			401 => "Unauthorized - invalid API key for OpenWeatherMap",
			404 => "City not found",
			429 => "Rate limit exceeded - too many requests - try again later",
			500..=599 => "OpenWeatherMap server error - try again later",
			_ => "Request failed",
		};
		return Err(eyre!("{error_message} (HTTP {status})"));
	}

	let data = res.json::<WeatherResponse>()?;
	Ok(data)
}
