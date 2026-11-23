# tsc

TypeScript compiler.

## Usage
- `tsc --init` : initialize a TypeScript project
- `tsc path/to/file.ts` : compile a TypeScript file into a JavaScript file with the same name
- `tsc -t ES5|ES2015|ES2016|ES2017|ES2018|ESNEXT|... path/to/file.ts` : compile a TypeScript file into JavaScript using a specific target syntax (default is `ES3`)
- `tsc --outFile path/to/output_file.js path/to/input_file.ts` : compile a TypeScript file into a JavaScript file with a custom name
- `tsc -b path/to/tsconfig.json` : compile all `.ts` files of a TypeScript project defined in a `tsconfig.json` file (`--build` can be omitted to build the project in the current working directory)
- `tsc @args.txt` : run the compiler using command-line options and arguments fetched from a text file
- `tsc --allowJs --checkJs --noEmit src/**/*.js` : type-check multiple JavaScript files, and output only the errors
- `tsc -w` : run the compiler in watch mode, which automatically recompiles code when it changes

## Categories
development, files, text

## Links
- More information: https://www.typescriptlang.org/docs/handbook/compiler-options.html
- tldr page: https://tldr.inbrowser.app/pages/common/tsc
