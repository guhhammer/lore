# aha

Convert ANSI escape sequences to HTML.

## Usage
- `color_command | aha >path/to/output.html` : convert terminal output to HTML
- `aha -f path/to/input.txt >path/to/output.html` : read from a file instead of `stdin`
- `color_command | aha -b >path/to/output.html` : convert to HTML with a black background
- `color_command | aha -p -t "title" >path/to/output.html` : convert to HTML with a pink background and set the document title
- `color_command | aha -c path/to/style.css >path/to/output.html` : add a CSS file to the HTML output
- `color_command | aha -w >path/to/output.html` : enable word-wrap to prevent horizontal scrolling
- `aha -h` : display help

## Categories
files

## Links
- More information: https://manned.org/aha
- tldr page: https://tldr.inbrowser.app/pages/common/aha
