# tuc

Cut text (or bytes) where a delimiter matches, then keep the desired parts. A more user-friendly and powerful version of `cut` with sensible defaults.

## Usage
- `echo "foo bar baz" | tuc -d ' ' -f 3,2,1` : cut and rearrange fields
- `echo "foo bar baz" | tuc -d ' ' -r ' ➡ '` : replace the delimiter `space` with an arrow
- `echo "foo bar    baz" | tuc -d ' ' -f 2:` : keep a range of fields
- `echo "a,b, c" | tuc -e '[, ]+' -f 1,3` : cut using `regex`
- `echo "foo bar baz" | tuc -d ' ' --json` : emit JSON output

## Categories
text, users

## Links
- More information: https://github.com/riquito/tuc#help
- tldr page: https://tldr.inbrowser.app/pages/common/tuc
