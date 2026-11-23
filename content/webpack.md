# webpack

Bundle a web project's js files and other assets into a single output file.

## Usage
- `webpack app.js bundle.js` : create a single output file from an entry point file
- `webpack app.js bundle.js --module-bind 'css=css'` : load CSS files too from the JavaScript file (this uses the CSS loader for CSS files)
- `webpack -c webpack.config.js --progress` : pass a configuration file (with e.g. the entry script and the output filename) and show compilation progress
- `webpack -w app.js bundle.js` : automatically recompile on changes to project files

## Categories
files, development

## Links
- More information: https://webpack.js.org/api/cli/
- tldr page: https://tldr.inbrowser.app/pages/common/webpack
