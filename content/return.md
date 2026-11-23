# return

Exit a function. Can exit out of a script if run with `source`.

## Usage
- `func_name() { echo "This is reached"; return; echo "This is not"; }` : exit a function prematurely
- `func_name() { return exit_code; }` : specify the function's return value

## See also
- exit

## Categories
development, shell

## Links
- More information: https://www.gnu.org/software/bash/manual/bash.html#index-return
- tldr page: https://tldr.inbrowser.app/pages/common/return
