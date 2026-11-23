# pup

HTML parsing tool.

## Usage
- `cat index.html | pup --color` : transform a raw HTML file into a cleaned, indented, and colored format
- `cat index.html | pup 'tag'` : filter HTML by element tag name
- `cat index.html | pup 'div#id'` : filter HTML by ID
- `cat index.html | pup 'input[type="text"]'` : filter HTML by attribute value
- `cat index.html | pup 'div text{}'` : print all text from the filtered HTML elements and their children
- `cat index.html | pup 'div json{}'` : print HTML as JSON

## Categories
text

## Links
- More information: https://github.com/ericchiang/pup
- tldr page: https://tldr.inbrowser.app/pages/common/pup
