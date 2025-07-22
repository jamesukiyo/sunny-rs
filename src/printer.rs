use super::{HeaderFooter, Output, OutputStyle, PrintOpts};
use colored::Colorize;
use console::{Alignment, Emoji, pad_str, pad_str_with};

const fn calc_f(temp: i64) -> i64 {
	temp * 9 / 5 + 32
}

impl HeaderFooter {
	fn show_header(&self) -> bool {
		matches!(self, HeaderFooter::Header | HeaderFooter::Both)
	}

	fn show_footer(&self) -> bool {
		matches!(self, HeaderFooter::Footer | HeaderFooter::Both)
	}
}

static COLOURS: [(u8, u8, u8); 11] = [
	(255, 255, 0),   // bright sunny yellow
	(255, 240, 50),  // light yellow
	(255, 220, 100), // golden yellow
	(255, 200, 120), // light orange
	(255, 180, 140), // peach
	(255, 160, 160), // coral pink
	(255, 140, 180), // pink
	(240, 120, 200), // rose pink
	(220, 100, 220), // light purple
	(200, 100, 240), // lavender purple
	(180, 120, 250), // light summer purple
];

fn create_box_line(
	content: &str,
	width: usize,
	colour: (u8, u8, u8),
) -> impl std::fmt::Display {
	format!("│{}│", pad_str(content, width, Alignment::Center, None))
		.truecolor(colour.0, colour.1, colour.2)
}

fn data_row(label: &str, value: &str, colour: (u8, u8, u8)) -> String {
	let l = "│   ".truecolor(colour.0, colour.1, colour.2);
	let m = format!(
		"{:12} {:>21}   ",
		label.white(),
		value.truecolor(colour.0, colour.1, colour.2)
	);
	let r = "│".truecolor(colour.0, colour.1, colour.2);
	format!("{l}{m}{r}")
}

#[allow(clippy::too_many_lines)]
pub fn printer(opts: &PrintOpts, output: &Output) {
	let (weather_icon, fallback_icon) = if opts.use_icons {
		match output.icon.as_str() {
			"01d" => ("☀️", ""), // clear sky day
			"01n" => ("☽", ""),  // clear sky night
			"02d" | "02n" | "03d" | "03n" | "04d" | "04n" | "50d" | "50n" => {
				("☁", "")
			} // cloudy/fog/mist
			"09d" | "09n" | "10d" | "10n" => ("☔", ""), // rain
			"11d" | "11n" => ("⚡", ""), // storm
			"13d" | "13n" => ("❄", ""), // snow
			_ => ("?", "?"),
		}
	} else {
		("?", "?")
	};

	let (temp_display, feels_like_display, unit) = if opts.use_fahrenheit {
		(calc_f(output.temp), calc_f(output.feels_like), "°F")
	} else {
		(output.temp, output.feels_like, "°C")
	};

	match opts.output_style {
		OutputStyle::Raw => {
			print!("{output:?}");
		}
		OutputStyle::Simple => {
			if opts.header_footer.show_header() {
				println!("\nsunny-rs\n");
			}
			if opts.use_icons {
				println!(
					"{}, {} {}",
					output.city,
					output.country,
					Emoji(weather_icon, fallback_icon)
				);
			} else {
				println!("{}, {}", output.city, output.country);
			}
			println!("Temperature: {temp_display}{unit}");
			println!("Feels like: {feels_like_display}{unit}");
			println!("Humidity: {}%", output.humidity);
			println!("Weather: {}", output.type_of.to_lowercase());
			println!("Description: {}", output.description);
			if opts.header_footer.show_footer() {
				println!("\nby github/jamesukiyo");
			}
		}
		OutputStyle::Pretty => {
			if opts.header_footer.show_header() {
				let header_content = if opts.use_icons {
					format!(
						" {} {} {} ",
						Emoji("⛅", ""),
						"sunny-rs".white(),
						Emoji("⛅", "")
					)
				} else {
					format!(" {} ", "sunny-rs".white())
				};
				println!(
					"{}",
					format!(
						"┌{}┐",
						pad_str_with(
							&header_content,
							40,
							Alignment::Center,
							None,
							'─'
						)
					)
					.truecolor(COLOURS[0].0, COLOURS[0].1, COLOURS[0].2)
				);
			} else {
				println!(
					"{}",
					format!(
						"┌{}┐",
						pad_str_with("", 40, Alignment::Center, None, '─')
					)
					.truecolor(COLOURS[0].0, COLOURS[0].1, COLOURS[0].2)
				);
			}

			println!("{}", create_box_line("", 40, COLOURS[1]));

			let city_info = if opts.use_icons {
				format!(
					"{}",
					format!(
						"{}, {} {}",
						output.city,
						output.country,
						Emoji(weather_icon, fallback_icon)
					)
					.blue()
				)
			} else {
				format!(
					"{}",
					format!("{}, {}", output.city, output.country).blue()
				)
			};
			println!("{}", create_box_line(&city_info, 40, COLOURS[2]));
			println!("{}", create_box_line("", 40, COLOURS[3]));
			println!(
				"{}",
				data_row(
					"Temperature:",
					&format!("{temp_display}{unit}"),
					COLOURS[4]
				)
			);
			println!(
				"{}",
				data_row(
					"Feels like:",
					&format!("{feels_like_display}{unit}"),
					COLOURS[5]
				)
			);
			println!(
				"{}",
				data_row(
					"Humidity:",
					&format!("{}%", output.humidity),
					COLOURS[6]
				)
			);
			println!(
				"{}",
				data_row(
					"Weather:",
					&format!("{:.15}", output.type_of.to_lowercase()),
					COLOURS[7]
				)
			);
			println!(
				"{}",
				data_row(
					"Description:",
					&format!("{:.15}", output.description),
					COLOURS[8]
				)
			);
			println!("{}", create_box_line("", 40, COLOURS[9]));

			let footer_content = if opts.header_footer.show_footer() {
				" by github/jamesukiyo "
			} else {
				""
			};
			println!(
				"{}",
				format!(
					"└{}┘",
					pad_str_with(
						footer_content,
						40,
						Alignment::Center,
						None,
						'─'
					)
				)
				.truecolor(COLOURS[10].0, COLOURS[10].1, COLOURS[10].2)
			);
		}
	}
}
