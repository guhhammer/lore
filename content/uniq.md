# uniq

Output the unique lines from an input or file. Since it does not detect repeated lines unless they are adjacent, we need to sort them first.

## Usage
- `sort path/to/file | uniq` : display each line once
- `sort path/to/file | uniq -u` : display only unique lines
- `sort path/to/file | uniq -d` : display only duplicate lines
- `sort path/to/file | uniq -i` : display each line once without case sensitivity
- `sort path/to/file | uniq -c` : display number of occurrences of each line along with that line
- `sort path/to/file | uniq -c | sort -nr` : display number of occurrences of each line, sorted by the most frequent
- `sort path/to/file | uniq -w 10` : compare only the first 10 characters on each line for uniqueness
- `sort path/to/file | uniq -s 5` : compare text after the first 5 characters on each line for uniqueness

## See also
- sort

## Categories
text

## Links
- More information: https://www.gnu.org/software/coreutils/manual/html_node/uniq-invocation.html
- tldr page: https://tldr.inbrowser.app/pages/common/uniq
