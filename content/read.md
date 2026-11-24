# read

Shell builtin for retrieving data from `stdin`.

## Usage
- `read variable1 variable2 ...` : store data that you type from the keyboard to one or more variable
- `read -a array` : store each of the next lines you enter as values of an array
- `read -n character_count variable` : specify the number of maximum characters to be read
- `read <<<"The surname is Bond" _ variable1 _ variable2` : assign multiple values to multiple variables
- `read -r variable` : do not let backslash (`\`) act as an escape character
- `read -p "Enter your input here: " variable` : display a prompt before the input
- `read -s variable` : do not echo typed characters (silent mode)
- `command | while IFS= read -r line; do echo|ls|rm|... "$line"; done` : perform an action on each line of a command's output

## Categories
shell, text

## Links
- More information: https://www.gnu.org/software/bash/manual/bash.html#index-read
- tldr page: https://tldr.inbrowser.app/pages/common/read
- Wikipedia: https://en.wikipedia.org/wiki/Read_(Unix)
