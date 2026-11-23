# gnuecho

This command is an alias of GNU `echo`.

## Usage
- `echo "Hello World"` : print a text message. Note: Quotes are optional
- `echo "My path is $PATH"` : print a message with environment variables
- `echo -n "Hello World"` : print a message without the trailing newline
- `echo "Hello World" >>file.txt` : append a message to the file
- `echo -e "Column 1\tColumn 2"` : enable interpretation of backslash escapes (special characters)
- `echo $?` : print the exit status of the last executed command (Note: In Windows Command Prompt and PowerShell the equivalent commands are `echo %errorlevel%` and `$lastexitcode` respectively)
- `echo "Hello World" | program` : pass text to another program through `stdin`

## Categories
documents, files, shell

## Links
- tldr page: https://tldr.inbrowser.app/pages/linux/gnuecho
