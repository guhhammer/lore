# tr

Translate characters: run replacements based on single characters and character sets.

## Usage
- `tr <path/to/file find_character replace_character` : replace all occurrences of a character in a file, and print the result
- `echo text | tr find_character replace_character` : replace all occurrences of a character from another command's output
- `tr <path/to/file 'abcd' 'jkmn'` : map each character of the first set to the corresponding character of the second set
- `tr <path/to/file -d 'input_characters'` : delete all occurrences of the specified set of characters from the input
- `tr <path/to/file -s 'input_characters'` : compress a series of identical characters to a single character
- `tr <path/to/file "[:lower:]" "[:upper:]"` : translate the contents of a file to upper-case
- `tr <path/to/file -cd "[:print:]"` : strip out non-printable characters from a file

## Categories
text

## Links
- More information: https://www.gnu.org/software/coreutils/manual/html_node/tr-invocation.html
- tldr page: https://tldr.inbrowser.app/pages/common/tr
