# tspin

A log file highlighter based on the `less` pager and basically behaves like any pager.

## Usage
- `tspin path/to/file.log` : view a log file using the default pager (`less`)
- `command | tspin` : read from another command and print to `stdout`
- `tspin path/to/file.log -p` : read from a file and print to `stdout` without paging
- `tspin -f path/to/file.log` : follow a file (mimics `tail -f`) and highlight new entries
- `tspin --enable urls,ip-addresses,... path/to/file.log` : highlight specific groups only (possible values: `numbers`, `urls`, `pointers`, `dates`, `paths`, `quotes`, `key-value-pairs`, `uuids`, `ip-addresses`, `processes`, `json`)
- `tspin --pager "bat -p [FILE]" path/to/file.log` : use a custom pager (the `[FILE]` string is a required literal placeholder)
- `tspin --highlight red:ERROR,WARNING,... path/to/file.log` : highlight custom strings with custom colors
- `tspin --exec='command'` : run the provided command and view the output in `less`

## Categories
files, monitoring

## Links
- More information: https://github.com/bensadeh/tailspin#usage
- tldr page: https://tldr.inbrowser.app/pages/common/tspin
