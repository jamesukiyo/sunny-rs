const { platform, arch } = require("os");
const path = require("path");
const { existsSync } = require("fs");

function getBinaryPath() {
	let platformName = platform();
	let archName = arch();
	let extension = "";

	// Map platform names
	if (platformName === "win32") {
		platformName = "windows";
		extension = ".exe";
	} else if (platformName === "darwin") {
		platformName = "darwin";
	} else if (platformName === "linux") {
		platformName = "linux";
	}

	// Map architecture names
	if (archName === "x64") {
		archName = "x64";
	} else if (archName === "arm64") {
		archName = "arm64";
	}

	const version = require("./package.json").version;
	const platformDir = `sunny-cli-${version}-${platformName}_${archName}`;
	const binaryPath = path.join(__dirname, platformDir, `sunny${extension}`);

	if (existsSync(binaryPath)) {
		return binaryPath;
	}

	throw new Error(
		`Unsupported platform: ${platformName}-${archName}. Binary not found at ${binaryPath}`
	);
}

module.exports = getBinaryPath();