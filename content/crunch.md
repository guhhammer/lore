# crunch

Wordlist generator.

## Usage
- `crunch 1 3` : output a list of words of length 1 to 3 with only lowercase characters
- `crunch 8 8 0123456789abcdef` : output a list of hexadecimal words of length 8
- `crunch 1 1 -p abc` : output a list of all permutations of abc (lengths are not processed)
- `crunch 1 1 -p abc def ghi` : output a list of all permutations of the given strings (lengths are not processed)
- `crunch 5 5 abcde123 -t @@@12 -d 2@` : output a list of words generated according to the given pattern and a maximum number of duplicate letters
- `crunch 3 5 -o START -b 10kb -s abc` : write a list of words in chunk files of a given size, starting with the given string
- `crunch 1 5 -o START -e abcde -i` : write a list of words stopping with the given string and inverting the wordlist
- `crunch 1 5 -o START -c 1000 -z gzip|bzip2|lzma|7z` : write a list of words in compressed chunk files with a specified number of words

## Categories
text

## Links
- More information: https://manned.org/crunch
- tldr page: https://tldr.inbrowser.app/pages/common/crunch
