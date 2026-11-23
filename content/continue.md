# continue

Skip to the next iteration of a `for`, `while`, `until`, or `select` loop.

## Usage
- `while :; do continue; echo "This will never be reached"; done` : skip to the next iteration
- `for i in {1..3}; do echo $i; while :; do continue 2; done; done` : skip to the next iteration from within a nested loop

## Categories
shell

## Links
- More information: https://www.gnu.org/software/bash/manual/bash.html#index-continue
- tldr page: https://tldr.inbrowser.app/pages/common/continue
