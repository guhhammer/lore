# ncc

Compile a Node.js application into a single file. Supports TypeScript, binary addons, and dynamic requires.

## Usage
- `ncc build path/to/file.js` : bundle a Node.js application
- `ncc build -m path/to/file.js` : bundle and minify a Node.js application
- `ncc build -s path/to/file.js` : bundle and minify a Node.js application and generate source maps
- `ncc build -w path/to/file.js` : automatically recompile on changes to source files
- `ncc run path/to/file.js` : bundle a Node.js application into a temporary directory and run it for testing
- `ncc clean cache` : clean the `ncc` cache

## Categories
files, development

## Links
- More information: https://github.com/vercel/ncc#usage
- tldr page: https://tldr.inbrowser.app/pages/common/ncc
