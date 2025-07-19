use clap::Parser;

#[derive(Parser)]
#[command(name = "sunny-rs")]
#[command(about = "A project by github/jamesukiyo\n\nView the weather from your terminal.", long_about = None)]
pub struct Args {
	/// City to get the weather for
	#[arg(index = 1, required = true)]
	pub city: String,

	/// Simpler output
	#[arg(short = 's', long = "simple")]
	pub simple: bool,
}
