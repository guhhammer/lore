# gnuuniq

This command is an alias of GNU `uniq`.

## Usage
- `sort path/to/file | uniq` : display each line once
- `sort path/to/file | uniq -u` : display only unique lines
- `sort path/to/file | uniq -d` : display only duplicate lines
- `sort path/to/file | uniq -i` : display each line once without case sensitivity
- `sort path/to/file | uniq -c` : display number of occurrences of each line along with that line
- `sort path/to/file | uniq -c | sort -nr` : display number of occurrences of each line, sorted by the most frequent
- `sort path/to/file | uniq -w 10` : compare only the first 10 characters on each line for uniqueness
- `sort path/to/file | uniq -s 5` : compare text after the first 5 characters on each line for uniqueness

## Categories
files, text, desktop

## Links
- tldr page: https://tldr.inbrowser.app/pages/linux/gnuuniq
