# esbuild

JavaScript bundler and minifier built for speed.

## Usage
- `esbuild --bundle path/to/file.js` : bundle a JavaScript application and print to `stdout`
- `esbuild <path/to/file.jsx --bundle --outfile=path/to/out.js` : bundle a JSX application from `stdin`
- `esbuild --bundle --define:process.env.NODE_ENV=\"production\" --minify --sourcemap path/to/file.js` : bundle and minify a JSX application with source maps in `production` mode
- `esbuild --bundle --minify --sourcemap --target=chrome58,firefox57,safari11,edge16 path/to/file.jsx` : bundle a JSX application for a comma-separated list of browsers
- `esbuild --bundle --platform=node --target=node12 path/to/file.js` : bundle a JavaScript application for a specific node version
- `esbuild --bundle app.js --loader:.js=jsx path/to/file.js` : bundle a JavaScript application enabling JSX syntax in `.js` files
- `esbuild --bundle --serve=port --outfile=index.js path/to/file.js` : bundle and serve a JavaScript application on an HTTP server
- `esbuild --bundle --outdir=path/to/output_directory path/to/file1 path/to/file2 ...` : bundle a list of files to an output directory

## Categories
files, network

## Links
- More information: https://esbuild.github.io/api/#general-options
- tldr page: https://tldr.inbrowser.app/pages/common/esbuild
