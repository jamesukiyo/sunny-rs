#!/usr/bin/env node

const { spawn } = require("child_process");

try {
	const binaryPath = require("./");
	const child = spawn(binaryPath, process.argv.slice(2), { stdio: "inherit" });
	
	child.on("exit", (code) => {
		process.exit(code || 0);
	});
} catch (error) {
	console.error(error.message);
	console.error("Visit https://github.com/jamesukiyo/sunny-rs for more information.");
	process.exit(1);
}
