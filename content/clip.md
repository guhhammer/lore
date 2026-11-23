# clip

Copy input content to the Windows clipboard.

## Usage
- `dir | clip` : pipe command-line output to the Windows clipboard
- `clip <path\to\file.ext` : copy the contents of a file to the Windows clipboard
- `echo some text | clip` : copy text with a trailing newline to the Windows clipboard
- `echo | set /p="some text" | clip` : copy text without a trailing newline to the Windows clipboard

## Notes
A Windows command: this page is for reference, it does not run on Linux or macOS.

## Categories
files, desktop, text, windows

## Links
- tldr page: https://tldr.inbrowser.app/pages/windows/clip
