# if

Perform conditional processing in shell scripts.

## Usage
- `if condition_command; then echo "Condition is true"; fi` : execute the specified commands if the condition command's exit status is zero
- `if ! condition_command; then echo "Condition is true"; fi` : execute the specified commands if the condition command's exit status is not zero
- `if condition_command; then echo "Condition is true"; else echo "Condition is false"; fi` : execute the first specified commands if the condition command's exit status is zero otherwise execute the second specified commands
- `if [[ -f path/to/file ]]; then echo "Condition is true"; fi` : check whether a file exists
- `if [[ -d path/to/directory ]]; then echo "Condition is true"; fi` : check whether a directory exists
- `if [[ -e path/to/file_or_directory ]]; then echo "Condition is true"; fi` : check whether a file or directory exists
- `if [[ -n "$variable" ]]; then echo "Condition is true"; fi` : check whether a variable is defined

## See also
- test
- [

## Categories
files, shell

## Links
- More information: https://www.gnu.org/software/bash/manual/bash.html#Conditional-Constructs
- tldr page: https://tldr.inbrowser.app/pages/common/if
