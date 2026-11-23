# local

Declare local variables and give them attributes.

## Usage
- `local variable="value"` : declare a string variable with the specified value
- `local -i variable="value"` : declare an integer variable with the specified value
- `local variable=(item_a item_b item_c)` : declare an array variable with the specified value
- `local -A variable=([key_a]=item_a [key_b]=item_b [key_c]=item_c)` : declare an associative array variable with the specified value
- `local -r variable="value"` : declare a readonly variable with the specified value
- `local --help` : display help

## See also
- declare
- export

## Categories
shell

## Links
- More information: https://www.gnu.org/software/bash/manual/bash.html#index-local
- tldr page: https://tldr.inbrowser.app/pages/common/local
