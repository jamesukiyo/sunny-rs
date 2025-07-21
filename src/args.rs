use clap::Parser;

#[allow(clippy::doc_markdown)]
#[derive(Parser)]
#[command(name = "sunny-rs")]
#[command(about = "A project by github/jamesukiyo\n\nView the weather from your terminal.", long_about = None)]
pub struct Args {
	/// City to get the weather for (overrides config)
	#[arg(index = 1, default_value = "")]
	pub city: String,

	/// Simpler output (no colours)
	#[arg(short = 's', long = "simple")]
	pub simple: bool,

	/// Raw JSON output
	#[arg(long = "raw")]
	pub raw: bool,

	/// API key for OpenWeatherMap (overrides config)
	#[arg(short = 'k', long = "key", default_value = "")]
	pub key: String,

	/// Use fahrenheit for temperature
	#[arg(short = 'f', long = "fahrenheit")]
	pub fahrenheit: bool,
}
