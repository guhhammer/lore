# spindump

Profile entire system during a time interval. Used for normal application force quits to display a dialog offering details and the option to report to Apple.

## Usage
- `sudo spindump` : sample all processes for 10 seconds with 10 milliseconds of run time between samples
- `sudo spindump pid|partial_name duration_seconds interval_milliseconds` : sort a specific process topmost for a custom duration and interval
- `sudo spindump pid|partial_name -o path/to/output.txt` : write output to a specific file

## Categories
processes, desktop, users, macos

## Links
- More information: https://keith.github.io/xcode-man-pages/spindump.8.html
- tldr page: https://tldr.inbrowser.app/pages/osx/spindump
