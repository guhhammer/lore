# rbash

This command is an alias of `bash --restricted`.

## Usage
- `bash` : start an interactive shell session
- `bash --norc` : start an interactive shell session without loading startup configs
- `bash -c "echo 'bash is executed'"` : execute specific commands
- `bash path/to/script.sh` : execute a specific script
- `bash -x path/to/script.sh` : execute a specific script, printing each command before executing it
- `bash -e path/to/script.sh` : execute a specific script and stop at the first error
- `echo "echo 'bash is executed'" | bash` : execute specific commands from `stdin`
- `bash -r` : start a restricted shell session

## Categories
development, shell, files

## Links
- tldr page: https://tldr.inbrowser.app/pages/common/rbash
