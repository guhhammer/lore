# lid.idutils

Query an ID database for tokens matching a pattern. Note: An ID database must first be built using `mkid`.

## Usage
- `lid` : list all tokens and their file locations in the ID database
- `lid token` : find files containing a specific token
- `lid -i token` : find tokens matching a pattern, ignoring case
- `lid -r "pattern"` : find tokens matching an extended `regex`
- `lid -R grep token` : output matching lines in grep-style format
- `lid -F 1` : find tokens that appear only once (useful for finding unused definitions)

## Categories
databases, text

## Links
- More information: https://www.gnu.org/software/idutils/manual/idutils.html#lid-invocation
- tldr page: https://tldr.inbrowser.app/pages/linux/lid.idutils
