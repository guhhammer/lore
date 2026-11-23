# ts

Add timestamps to every line from `stdin`.

## Usage
- `command | ts` : add a timestamp to the beginning of each line
- `command | ts "%b %d %H:%M:%.S"` : add timestamps with microsecond precision
- `command | ts -i "%H:%M:%.S"` : add incremental timestamps with microsecond precision, starting from zero
- `cat path/to/file | ts -r` : convert existing timestamps in a text file (e.g. a log file) into relative format

## Categories
text, files

## Links
- More information: https://manned.org/ts
- tldr page: https://tldr.inbrowser.app/pages/common/ts
