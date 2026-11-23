# zformat

Format strings in Zsh. This builtin is part of the `zsh/zutil` module.

## Usage
- `zmodload zsh/zutil` : load the zformat module
- `zformat -f variable "Hello %c" c:world` : format a string by replacing `%c` with a value and store the result in a variable
- `zformat -f variable "%10c" c:hello` : format with right-padding (left-align) to a minimum width
- `zformat -f variable "%-10c" c:hello` : format with left-padding (right-align) using negative width
- `zformat -f variable "The answer is '%3(c.yes.no)'." c:3` : use ternary expression for conditional text (if value is 3, outputs "yes", otherwise "no")
- `zformat -f variable "name: %-15n value: %-10v" n:value1 v:value2` : format with left-padding (right-align) using negative minimum width
- `zformat -a array : left1:right1 left2:right2` : align strings (left:right pairs separated by colon)

## See also
- zstyle

## Categories
development, system, shell

## Links
- More information: https://zsh.sourceforge.io/Doc/Release/Zsh-Modules.html
- tldr page: https://tldr.inbrowser.app/pages/common/zformat
