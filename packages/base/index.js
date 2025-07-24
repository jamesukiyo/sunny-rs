#!/usr/bin/env node

import { spawnSync } from "child_process";
import { createRequire } from "module";
import path from "path";
import fs from "fs";

const require = createRequire(import.meta.url);

/**
 * Returns the executable path which is located inside `node_modules`
 * The naming convention is app-${os}-${arch}
 * If the platform is `win32` or `cygwin`, executable will include a `.exe` extension.
 * @see https://nodejs.org/api/os.html#osarch
 * @see https://nodejs.org/api/os.html#osplatform
 * @example "x/xx/node_modules/app-darwin-arm64"
 */
function get_exe_path() {
	let arch = process.arch;
	let os = process.platform;
	let extension = "";

	// map platform to package naming
	if (["win32", "cygwin"].includes(process.platform)) {
		os = "windows";
		extension = ".exe";
	} else if (process.platform === "darwin") {
		os = "darwin";
	} else if (process.platform === "linux") {
		os = "linux";
	}

	// map arch to package naming
	if (arch === "x64") {
		arch = "x64";
	} else if (arch === "arm64") {
		arch = "arm64";
	}

	try {
		// try to resolve from installed platform package
		return require.resolve(
			`@jamesukiyo/sunny-cli-${os}-${arch}/sunny${extension}`,
		);
	} catch (e) {
		// if it fails check if binary was placed directly in our directory
		const __dirname = path.dirname(new URL(import.meta.url).pathname);
		const local_bin_path = path.join(__dirname, `sunny${extension}`);
		if (fs.existsSync(local_bin_path)) {
			return local_bin_path;
		}

		throw new Error(
			`Couldn't find application binary for ${os}-${arch}. Make sure @jamesukiyo/sunny-cli-${os}-${arch} is installed or run 'npm install @jamesukiyo/sunny-cli-${os}-${arch}'`,
		);
	}
}

// runs the application with args using nodejs spawn
function run() {
	const args = process.argv.slice(2);
	const process_result = spawnSync(get_exe_path(), args, {
		stdio: "inherit",
	});
	process.exit(process_result.status ?? 0);
}

run();
