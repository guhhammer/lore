# gxargs

This command is an alias of GNU `xargs`.

## Usage
- `arguments_source | xargs command` : run a command using the input data as arguments
- `arguments_source | xargs sh -c "command1 && command2 | command3"` : run multiple chained commands on the input data
- `arguments_source | xargs -n 1 command` : execute a new command with each argument
- `arguments_source | xargs -P 10 -n 1 command` : raise the parallel process limit to 10 (default is 1; 0 means as many processes as possible)
- `arguments_source | xargs -I _ command _ optional_extra_arguments` : execute the command once for each input line, replacing any occurrences of the placeholder (here marked as `_`) with the input line
- `arguments_source | xargs -p command` : prompt user for confirmation before executing command (confirm with `y` or `Y`)
- `xargs -a path/to/file command` : read a file for arguments to be given to a command
- `arguments_source | xargs -o command` : allow the command to access the terminal for interactive input

## Categories
shell, files, macos

## Links
- tldr page: https://tldr.inbrowser.app/pages/osx/gxargs
