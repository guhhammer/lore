# mcfly

A smart command history search and management tool. Replaces your default shell history search (`<Ctrl r>`) with an intelligent search engine providing context and relevance to the commands.

## Usage
- `mcfly init bash|fish|zsh` : print the mcfly integration code for the specified shell
- `mcfly search --results 20 "search_terms"` : search the history for a command, with 20 results
- `mcfly add "command"` : add a new command to the history
- `mcfly move "path/to/old_directory" "path/to/new_directory"` : record that a directory has moved and transfer the historical records from the old path to the new one
- `mcfly train` : train the suggestion engine (developer tool)
- `mcfly help subcommand` : display help for a specific subcommand

## Categories
shell, files

## Links
- More information: https://github.com/cantino/mcfly
- tldr page: https://tldr.inbrowser.app/pages/common/mcfly
