# goaccess

An open source real-time web log analyzer.

## Usage
- `goaccess path/to/log1 path/to/log2 ...` : analyze one or more log files in interactive mode
- `goaccess path/to/logfile --log-format=format` : use a specific log-format (or pre-defined formats like "combined")
- `tail -f path/to/logfile | goaccess -` : analyze a log from `stdin`
- `goaccess path/to/logfile -o path/to/file.html --real-time-html` : analyze a log and write it to an HTML file in real-time

## Categories
files, monitoring

## Links
- More information: https://goaccess.io/man
- tldr page: https://tldr.inbrowser.app/pages/linux/goaccess
