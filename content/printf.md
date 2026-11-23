# printf

Format and print text.

## Usage
- `printf "%s\n" "Hello world"` : print a text message
- `printf "%s\n" string1 string2 ...` : print formatted message multiple times with different strings
- `printf "\e[1;34m%.3d\e[0m\n" 42` : print an integer in bold blue
- `printf "\u20AC %.2f\n" 123.4` : print a float number with the Unicode Euro sign
- `printf "var1: %s\tvar2: %s\n" "$VAR1" "$VAR2"` : print a text message composed with environment variables
- `printf -v myvar "This is %s = %d\n" "a year" 2016` : store a formatted message in a variable (does not work on Zsh)
- `printf "hex=%x octal=%o scientific=%e\n" 0xFF 0377 100000` : print a hexadecimal, octal, and scientific number

## See also
- echo

## Categories
documents, text, shell

## Links
- tldr page: https://tldr.inbrowser.app/pages/common/printf
