# jj util

Infrequently used utility commands for Jujutsu.

## Usage
- `jj util completion bash|fish|nushell|zsh` : generate shell completion script
- `jj util exec -- command` : execute an external command in the workspace (helpful for creating aliases)
- `jj util gc` : run garbage collection on obsolete objects and abandoned operations older than 2 weeks
- `jj util gc --expire now` : run garbage collection immediately without waiting for the default expiration
- `jj util snapshot` : explicitly snapshot the working copy
- `jj util config-schema` : print the JSON schema for the Jujutsu configuration format
- `jj util backend name` : print the name of the backend used in the current repository
- `jj util install-man-pages path/to/directory` : install Jujutsu's man pages to a directory

## Categories
files, packages

## Links
- More information: https://docs.jj-vcs.dev/latest/cli-reference/#jj-util
- tldr page: https://tldr.inbrowser.app/pages/common/jj-util
