# jj bisect

Find a bad revision by bisection.

## Usage
- `jj bisect run -r good_revision..bad_revision command` : find the first bad revision in a range by running a test command
- `jj bisect run -r good_revision..bad_revision -- bash -c "command"` : find the first bad revision using a shell command
- `jj bisect run -r good_revision..bad_revision --find-good command` : find the first good revision instead of the first bad one
- `jj bisect run -r good_revision..bad_revision --find-good -- test -f path/to/file` : find the first revision where a file was added

## Categories
files

## Links
- More information: https://docs.jj-vcs.dev/latest/cli-reference/#jj-bisect
- tldr page: https://tldr.inbrowser.app/pages/common/jj-bisect
