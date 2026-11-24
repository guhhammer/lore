# sort and remove duplicate lines

Sort lines, keep each once, count how often each appears.

## Steps
- `sort file.txt` : sort the lines
- `sort -u file.txt` : sort them and keep each once
- `sort file.txt | uniq -c | sort -rn` : how often each line appears, most first
- `awk '!seen[$0]++' file.txt` : keep each once, in their order
- `sort -t, -k3 -n data.csv` : sort a CSV by its third column, as numbers
- `sort -h sizes.txt` : sort sizes like 2K, 1M, 3G

## Notes
- uniq only removes repeats next to each other: sort first.

## See also
- sort
- uniq
- awk

## Categories
text
