# deno

A secure runtime for JavaScript, TypeScript, and WebAssembly. Includes dependency management using `npm` or `jsr`, and tooling like bench, bundle, doc, and coverage.

## Usage
- `deno` : start a REPL (interactive shell, also known as Read-Eval-Print Loop)
- `deno init sample && cd sample && deno test` : start a new project named sample and test it
- `deno run path/to/file.ts` : run a file securely. It will ask (if needed) to allow net, read, etc
- `deno run --allow-env|--allow-net|--allow-write|--allow-all jsr:@deno/deployctl` : run a file with explicit permissions or allow all (only if you trust the source)
- `deno task` : list and run tasks from `deno.json` or scripts from `package.json`
- `deno install` : install dependencies listed in `deno.json` or `package.json` (also lock files)
- `deno check && deno fmt && deno lint --fix` : check types, format, and lint (fix if possible)
- `deno compile path/to/file.ts` : compile the script, imported dependencies, and runtime into a self contained executable

## Categories
development, files, packages

## Links
- More information: https://docs.deno.com/runtime/reference/cli/
- tldr page: https://tldr.inbrowser.app/pages/common/deno
