use super::Output;
use colored::Colorize;

#[rustfmt::skip]
pub fn printer(raw: bool, simple: bool, output: &Output) {
	// colour palette
    let c = [
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

    let weather_icon = match output.icon.as_str() {
        "01d" => "☀", // clear sky day
        "01n" => "☽", // clear sky night
        "02d" | "02n" | "03d" | "03n" | "04d" | "04n" | "50d" | "50n" => "☁", // cloudy/fog/mist
        "09d" | "09n" | "10d" | "10n" => "☔", // rain
        "11d" | "11n" => "⚡", // storm
        "13d" | "13n" => "❄", // snow
        _ => "?",
    };

	if raw {
		print!("{output:?}")
	} else if simple {
		println!("\nsunny-rs\n");
		println!("{}, {} {}", output.city, output.country, weather_icon);
		println!("Temperature: {}°C", output.temp_c);
		println!("Feels like: {}°C", output.feels_like_c);
		println!("Humidity: {}%", output.humidity);
		println!("Weather: {}", output.type_of.to_lowercase());
		println!("Description: {}", output.description);
		println!("\nby github/jamesukiyo");
	} else {
		// sunny-rs
		println!("{}", format!("┌{:─^47}┐", format!(" ⛅ {} ⛅ ", "sunny-rs".white())).truecolor(c[0].0, c[0].1, c[0].2));
		// blank
		println!("{}", format!("│{: ^40}│", "").truecolor(c[1].0, c[1].1, c[1].2));
		// city, country, weather icon
		println!("{}", format!("│{: ^40}│", format!("{}, {} {}", output.city, output.country, weather_icon).blue()).truecolor(c[2].0, c[2].1, c[2].2));
		// blank
		println!("{}", format!("│{: ^40}│", "").truecolor(c[3].0, c[3].1, c[3].2));
		// temperature
		println!("{}{}{}", "│   ".truecolor(c[4].0, c[4].1, c[4].2), format!("{:12} {:>21}   ", "Temperature:".white(), format!("{}°C",   output.temp_c)                .truecolor(c[4].0, c[4].1, c[4].2)), "│".truecolor(c[4].0, c[4].1, c[4].2));
		// feels like
		println!("{}{}{}", "│   ".truecolor(c[5].0, c[5].1, c[5].2), format!("{:12} {:>21}   ", "Feels like:".white(),  format!("{}°C",   output.feels_like_c)          .truecolor(c[5].0, c[5].1, c[5].2)), "│".truecolor(c[5].0, c[5].1, c[5].2));
		// humidity
		println!("{}{}{}", "│   ".truecolor(c[6].0, c[6].1, c[6].2), format!("{:12} {:>21}   ", "Humidity:".white(),    format!("{}%",    output.humidity)              .truecolor(c[6].0, c[6].1, c[6].2)), "│".truecolor(c[6].0, c[6].1, c[6].2));
		// weather
		println!("{}{}{}", "│   ".truecolor(c[7].0, c[7].1, c[7].2), format!("{:12} {:>21}   ", "Weather:".white(),     format!("{:.15}", output.type_of.to_lowercase()).truecolor(c[7].0, c[7].1, c[7].2)), "│".truecolor(c[7].0, c[7].1, c[7].2));
		// description
		println!("{}{}{}", "│   ".truecolor(c[8].0, c[8].1, c[8].2), format!("{:12} {:>21}   ", "Description:".white(), format!("{:.15}", output.description)           .truecolor(c[8].0, c[8].1, c[8].2)), "│".truecolor(c[8].0, c[8].1, c[8].2));
		// blank
		println!("{}", format!("│{: ^40}│", "").truecolor(c[9].0, c[9].1, c[9].2));
		// by jamesukiyo
		println!("{}", format!("└{:─^40}┘", " by github/jamesukiyo ").truecolor(c[10].0, c[10].1, c[10].2));
	}
}
