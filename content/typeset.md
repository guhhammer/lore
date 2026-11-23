# typeset

This command is an alias of `declare`.

## Usage
- `declare variable="value"` : declare a string variable with the specified value
- `declare -i variable="value"` : declare an integer variable with the specified value
- `declare -a variable=(item_a item_b item_c)` : declare an array variable with the specified value
- `declare -A variable=([key_a]=item_a [key_b]=item_b [key_c]=item_c)` : declare an associative array variable with the specified value
- `declare -r variable="value"` : declare a readonly string variable with the specified value
- `declare -g variable="value"` : declare a global variable within a function with the specified value
- `declare -f function_name` : print a function definition
- `declare -p variable_name` : print a variable definition

## Categories
shell

## Links
- tldr page: https://tldr.inbrowser.app/pages/common/typeset
