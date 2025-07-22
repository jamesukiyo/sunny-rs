use clap::Parser;

#[allow(clippy::doc_markdown, clippy::struct_excessive_bools)]
#[derive(Parser)]
#[command(name = "sunny-rs")]
#[command(about = "A project by github/jamesukiyo\n\nView the current weather from your terminal.", long_about = None)]
pub struct Args {
	/// City to get the weather for
	#[arg(index = 1, default_value = "")]
	pub city: String,

	/// Simpler output (no styling)
	#[arg(short = 's', long = "simple")]
	pub simple: bool,

	/// Raw JSON output
	#[arg(short = 'r', long = "raw")]
	pub raw: bool,

	/// API key for OpenWeatherMap
	#[arg(short = 'k', long = "key", default_value = "")]
	pub key: String,

	/// Use fahrenheit for temperature
	#[arg(short = 'f', long = "fahrenheit")]
	pub fahrenheit: bool,

	/// Hide the credits footer from output
	#[arg(short = 'F', long = "no-footer")]
	pub no_footer: bool,

	/// Hide the header from output
	#[arg(short = 'H', long = "no-header")]
	pub no_header: bool,

	/// Alias for --no-header --no-footer
	#[arg(short = 'c', long = "clean")]
	pub clean_output: bool,

	/// Disable icons - good for non-nerd fonts or lack of emoji support
	#[arg(short = 'i', long = "no-icons")]
	pub no_icons: bool,
}
