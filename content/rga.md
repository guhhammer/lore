# rga

Ripgrep wrapper with rich file type searching capabilities.

## Usage
- `rga regex` : search recursively for a pattern in all files in the current directory
- `rga --rga-list-adapters` : list available adapters
- `rga --rga-adapters=adapter1,adapter2 regex` : change which adapters to use (e.g. `ffmpeg`, `pandoc`, `poppler` etc.)
- `rga --rga-accurate regex` : search for a pattern using the mime type instead of the file extension (slower)
- `rga --help` : display help

## Categories
files, text

## Links
- More information: https://github.com/phiresky/ripgrep-all
- tldr page: https://tldr.inbrowser.app/pages/common/rga
