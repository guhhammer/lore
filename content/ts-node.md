# ts-node

Run TypeScript code directly, without any compiling.

## Usage
- `ts-node path/to/file.ts` : execute a TypeScript file without compiling (Node + `tsc`)
- `ts-node --skipProject path/to/file.ts` : execute a TypeScript file without loading `tsconfig.json`
- `ts-node -e 'console.log("Hello World")'` : evaluate TypeScript code passed as a literal
- `ts-node --script-mode path/to/file.ts` : execute a TypeScript file in script mode
- `ts-node -T path/to/file.ts` : transpile a TypeScript file to JavaScript without executing it
- `ts-node -h` : display help

## Categories
files

## Links
- More information: https://typestrong.org/ts-node/docs/options/
- tldr page: https://tldr.inbrowser.app/pages/common/ts-node
