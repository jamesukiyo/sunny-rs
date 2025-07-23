#!/usr/bin/env node

import { spawnSync } from "child_process";

/**
 * Returns the executable path which is located inside `node_modules`
 * The naming convention is app-${os}-${arch}
 * If the platform is `win32` or `cygwin`, executable will include a `.exe` extension.
 * @see https://nodejs.org/api/os.html#osarch
 * @see https://nodejs.org/api/os.html#osplatform
 * @example "x/xx/node_modules/app-darwin-arm64"
 */
function get_exe_path() {
	const arch = process.arch;
	let os = process.platform;
	let extension = "";
	if (["win32", "cygwin"].includes(process.platform)) {
		os = "windows";
		extension = ".exe";
	}

	try {
		return require.resolve(
			`@jamesukiyo/sunny-cli-${os}-${arch}/bin/sunny${extension}`,
		);
	} catch (e) {
		throw new Error(
			`Couldn't find application binary inside node_modules for ${os}-${arch}`,
		);
	}
}

// Runs the application with args using nodejs spawn
function run() {
	const args = process.argv.slice(2);
	const process_result = spawnSync(get_exe_path(), args, {
		stdio: "inherit",
	});
	process.exit(process_result.status ?? 0);
}

run();
