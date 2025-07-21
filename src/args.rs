use clap::Parser;

#[derive(Parser)]
#[command(name = "sunny-rs")]
#[command(about = "A project by github/jamesukiyo\n\nView the weather from your terminal.", long_about = None)]
pub struct Args {
	/// City to get the weather for
	#[arg(index = 1, required = true)]
	pub city: String,

	/// Simpler output (no colours)
	#[arg(short = 's', long = "simple")]
	pub simple: bool,

	/// Raw JSON output
	#[arg(long = "raw")]
	pub raw: bool,

	/// API key for OpenWeatherMap
	#[arg(short = 'k', long = "key", default_value = "OPEN_WEATHER_API_KEY")]
	pub key: String,

	/// Use fahrenheit for temperature
	#[arg(short = 'f', long = "fahrenheit")]
	pub fahrenheit: bool,
}
