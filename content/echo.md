# echo

Print given arguments.

## Usage
- `echo "Hello World"` : print a text message. Note: Quotes are optional
- `echo "My path is $PATH"` : print a message with environment variables
- `echo -n "Hello World"` : print a message without the trailing newline
- `echo "Hello World" >>file.txt` : append a message to the file
- `echo -e "Column 1\tColumn 2"` : enable interpretation of backslash escapes (special characters)
- `echo $?` : print the exit status of the last executed command (Note: In Windows Command Prompt and PowerShell the equivalent commands are `echo %errorlevel%` and `$lastexitcode` respectively)
- `echo "Hello World" | program` : pass text to another program through `stdin`

## See also
- printf

## Categories
shell, text

## Links
- More information: https://www.gnu.org/software/coreutils/manual/html_node/echo-invocation.html
- tldr page: https://tldr.inbrowser.app/pages/common/echo
- Wikipedia: https://en.wikipedia.org/wiki/Echo_(command)
