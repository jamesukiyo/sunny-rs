use std::fs;
use std::path::PathBuf;

use color_eyre::eyre::{Result, eyre};
use serde::{Deserialize, Serialize};

#[allow(clippy::struct_excessive_bools)]
#[derive(Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct Config {
	pub city: String,
	pub api_key: String,
	pub use_fahrenheit: bool,
	pub show_footer: bool,
	pub show_header: bool,
	pub icons: bool,
	pub colours: bool,
}

impl Default for Config {
	fn default() -> Self {
		Self {
			city: String::new(),
			api_key: String::new(),
			use_fahrenheit: false,
			show_footer: true,
			show_header: true,
			icons: true,
			colours: true,
		}
	}
}

impl Config {
	pub fn load() -> Result<Self> {
		let config_path = Self::config_path()?;

		if !config_path.exists() {
			// create default config file
			let default_config = Config::default();
			default_config.save()?;
			return Ok(default_config);
		}

		let content = fs::read_to_string(&config_path).map_err(|e| {
			eyre!(
				"Failed to read config file at {}: {}",
				config_path.display(),
				e
			)
		})?;

		// fallback to default config
		match toml::from_str(&content) {
			Ok(config) => Ok(config),
			Err(e) => {
				eprintln!(
					"Warning: Config file at {} has invalid syntax: {}",
					config_path.display(),
					e
				);
				eprintln!(
					"Using default configuration. Fix the config file or delete it to regenerate."
				);
				Ok(Config::default())
			}
		}
	}

	pub fn save(&self) -> Result<()> {
		let config_path = Self::config_path()?;

		// create config directory if it doesnt exist
		if let Some(parent) = config_path.parent() {
			fs::create_dir_all(parent)?;
		}

		// convert config to toml string
		let content = toml::to_string_pretty(self)?;
		fs::write(&config_path, content)?;
		Ok(())
	}

	fn config_path() -> Result<PathBuf> {
		let home = std::env::var("HOME")
			.or_else(|_| std::env::var("USERPROFILE"))
			.map_err(|_| eyre!("Could not find home directory"))?;

		Ok(PathBuf::from(home).join(".config").join("sunny.toml"))
	}
}
