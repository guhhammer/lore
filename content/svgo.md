# svgo

SVG Optimizer: optimizing Scalable Vector Graphics files. Based in Node.js. It applies a series of transformation rules (plugins), which can be toggled individually.

## Usage
- `svgo test.svg` : optimize a file using the default plugins (overwrites the original file)
- `svgo test.svg -o test.min.svg` : optimize a file and save the result to another file
- `svgo -f path/to/directory_with_svg_files` : optimize all SVG files within a directory (overwrites the original files)
- `svgo -f path/to/input_directory -o path/to/output_directory` : optimize all SVG files within a directory and save the resulting files to another directory
- `cat test.svg | svgo -i - -o test.min.svg` : optimize SVG content passed from another command, and save the result to a file
- `svgo test.svg -o -` : optimize a file and print out the result
- `svgo --show-plugins` : show available plugins

## Categories
files, development

## Links
- More information: https://manned.org/svgo
- tldr page: https://tldr.inbrowser.app/pages/common/svgo
