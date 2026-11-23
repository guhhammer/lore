# shuf

Generate random permutations.

## Usage
- `shuf path/to/file` : randomize the order of lines in a file and output the result
- `shuf -n 5 path/to/file` : only output the first 5 entries of the result
- `shuf path/to/input_file -o path/to/output_file` : write the output to another file
- `shuf -n 3 -i 1-10 -r` : generate 3 random numbers in the range 1-10 (inclusive, numbers can repeat)

## Categories
files

## Links
- More information: https://www.gnu.org/software/coreutils/manual/html_node/shuf-invocation.html
- tldr page: https://tldr.inbrowser.app/pages/common/shuf
