# bun build

Bundle JavaScript and TypeScript files with Bun's fast native bundler.

## Usage
- `bun build path/to/entry.ts --outfile path/to/output.js` : bundle an entry point to a single output file
- `bun build path/to/entry1.ts path/to/entry2.ts ... --outdir path/to/dist` : bundle multiple entry points to an output directory
- `bun build path/to/entry.ts --outfile path/to/output.js --sourcemap` : bundle with source maps for debugging
- `bun build path/to/entry.ts --outfile path/to/output.js --minify` : bundle with minification for production
- `bun build path/to/entry.ts --outfile path/to/output.js --target browser|bun|node` : bundle with a specific target environment
- `bun build path/to/entry.ts --compile --outfile path/to/executable` : bundle to a standalone executable
- `bun build path/to/entry.ts --outfile path/to/output.js --watch` : watch for file changes and rebuild automatically
- `bun build path/to/entry.ts --outfile path/to/output.js -e react react-dom` : bundle with external dependencies not included in the output

## Categories
files, development

## Links
- More information: https://bun.com/docs/bundler
- tldr page: https://tldr.inbrowser.app/pages/common/bun-build
- Wikipedia: https://en.wikipedia.org/wiki/Bun_(software)
