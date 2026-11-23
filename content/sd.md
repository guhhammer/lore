# sd

Intuitive find and replace.

## Usage
- `echo 'lorem ipsum 23   ' | sd '\s+$' ''` : trim some whitespace using a `regex` (output stream: `stdout`)
- `echo 'cargo +nightly watch' | sd '(\w+)\s+\+(\w+)\s+(\w+)' 'cmd: $1, channel: $2, subcmd: $3'` : replace words using capture groups (output stream: `stdout`)
- `sd -p 'window.fetch' 'fetch' path/to/file.js` : find and replace in a specific file (output stream: `stdout`)
- `find . -type f -exec sd 'from "react"' 'from "preact"' {} \;` : find and replace in all files in the current project

## Categories
files, media

## Links
- More information: https://manned.org/sd
- tldr page: https://tldr.inbrowser.app/pages/common/sd
