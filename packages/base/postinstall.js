#!/usr/bin/env node

const { execSync } = require("child_process");
const { existsSync } = require("fs");
const path = require("path");

function get_platform_pkg_name() {
	let arch = process.arch;
	let os = process.platform;

	// Map platform to package naming
	if (["win32", "cygwin"].includes(process.platform)) {
		os = "windows";
	} else if (process.platform === "darwin") {
		os = "darwin";
	} else if (process.platform === "linux") {
		os = "linux";
	}

	// Map arch to package naming
	if (arch === "x64") {
		arch = "x64";
	} else if (arch === "arm64") {
		arch = "arm64";
	}

	return `@jamesukiyo/sunny-cli-${os}-${arch}`;
}

function install_platform_bin() {
	const packageName = get_platform_pkg_name();
	console.log(`Installing platform-specific binary: ${packageName}`);

	try {
		// Install the platform-specific package
		execSync(`npm install ${packageName}`, {
			stdio: "inherit",
			cwd: __dirname,
		});

		console.log(`Successfully installed ${packageName}`);
	} catch (error) {
		console.error(`Failed to install ${packageName}:`, error.message);
		console.error("The CLI may not work properly on this platform");
		process.exit(0); // Don't fail the main package installation
	}
}

// only run if this is being executed directly (not required)
if (require.main === module) {
	install_platform_bin();
}

module.exports = { get_platform_pkg_name, install_platform_bin };
