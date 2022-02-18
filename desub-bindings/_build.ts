#!/usr/bin/env -S deno run -A

import * as path from "https://deno.land/std@0.119.0/path/mod.ts";

const OUT_DIR = "target/deno";
function relativeToOutDir(subpath: string) {
	return path.join(OUT_DIR, subpath);
}

async function run(cmd: string[], cwd?: string): Promise<void> {
	let p = Deno.run({
		cmd,
		stdin: "inherit",
		stdout: "inherit",
		stderr: "inherit",
		cwd,
	});
	try {
		const status = await p.status();
		if (!status.success) {
			console.error(status);
			Deno.exit(1);
		}
	} finally {
		p.close();
	}
}

await run(["cargo", "build", "--target", "wasm32-unknown-unknown"], new URL(path.dirname(import.meta.url)).pathname);
await Promise.all(
	["deno", "web", "nodejs"].map((envName) => {
		const outDir = path.join("desub-bindings", envName);
		const outFile = path.join(outDir, "desub_bindings_bg.wasm");
		return [
			[
				"wasm-bindgen",
				path.join("target", "wasm32-unknown-unknown", "debug", "desub_bindings.wasm"),
				"--target",
				envName,
				"--weak-refs",
				"--out-dir",
				outDir,
			],
			["wasm-opt", "-g", "-Oz", outFile, "-o", outFile],
		].map((cmd) => run(cmd));
	})
);
