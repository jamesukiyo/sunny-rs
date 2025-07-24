use color_eyre::eyre::{Result, eyre};
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct WeatherResponse {
	pub name: String,
	pub main: Main,
	pub sys: Sys,
	pub weather: Vec<Weather>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Weather {
	pub main: String,
	pub description: String,
	pub icon: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Main {
	pub temp: f64,
	pub feels_like: f64,
	pub humidity: i32,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Sys {
	pub country: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ForecastResponse {
	pub city: ForecastCity,
	pub list: Vec<ForecastItem>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ForecastCity {
	pub name: String,
	pub country: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ForecastItem {
	pub dt: i64,
	pub main: Main,
	pub weather: Vec<Weather>,
	pub dt_txt: String,
}

pub fn fetch_weather(
	city: &str,
	api_key: &str,
	is_tomorrow: bool,
) -> Result<WeatherResponse> {
	let endpoint = if is_tomorrow { "forecast" } else { "weather" };
	let url = format!(
		"https://api.openweathermap.org/data/2.5/{endpoint}?q={city}&appid={api_key}"
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

	if is_tomorrow {
		let forecast_data = res.json::<ForecastResponse>()?;
		let tomorrow_forecast = forecast_data
			.list
			.get(8)
			.ok_or_else(|| eyre!("No forecast data available for tomorrow"))?;

		Ok(WeatherResponse {
			name: forecast_data.city.name,
			main: Main {
				temp: tomorrow_forecast.main.temp,
				feels_like: tomorrow_forecast.main.feels_like,
				humidity: tomorrow_forecast.main.humidity,
			},
			sys: Sys {
				country: forecast_data.city.country,
			},
			weather: tomorrow_forecast.weather.clone(),
		})
	} else {
		Ok(res.json::<WeatherResponse>()?)
	}
}
