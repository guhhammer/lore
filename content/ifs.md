# ifs

`$IFS` (Internal Field Separator) is a special environment variable that defines the delimiter used for word splitting in Unix shells. The default value of `$IFS` is a space, tab, and newline. The three characters serve as delimiters.

## Usage
- `echo "$IFS"` : view the current `$IFS` value
- `IFS=":"` : change the `$IFS` value
- `IFS=: read variable1 variable2 ...` : specify what character to use for word splitting with `read`
- `IFS=$' \t\n'` : reset `$IFS` to default
- `(IFS=":"; echo "one:two:three")` : temporarily change the `$IFS` value in a subshell

## Categories
text, shell

## Links
- More information: https://www.gnu.org/software/bash/manual/bash.html#Word-Splitting
- tldr page: https://tldr.inbrowser.app/pages/common/ifs
