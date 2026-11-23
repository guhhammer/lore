# htmlq

Use CSS selectors to extract content from HTML files.

## Usage
- `cat path/to/file.html | htmlq '.card'` : return all elements of class `card`
- `cat path/to/file.html | htmlq -t 'p:first-of-type'` : get the text content of the first paragraph
- `cat path/to/file.html | htmlq -a href 'a'` : find all the links in a page
- `cat path/to/file.html | htmlq -r 'img' -r 'svg'` : remove all images and SVGs from a page
- `htmlq -p -f path/to/input.html -o path/to/output.html` : pretty print and write the output to a file

## Categories
files, archives

## Links
- More information: https://github.com/mgdm/htmlq#usage
- tldr page: https://tldr.inbrowser.app/pages/common/htmlq
