# fswatch

A cross-platform file change monitor.

## Usage
- `fswatch path/to/file | xargs -n 1 bash_command` : run a Bash command on file creation, update, or deletion
- `fswatch path/to/file path/to/directory path/to/another_directory/**/*.js | xargs -n 1 bash_command` : watch one or more files and/or directories
- `fswatch path/to/directory | xargs -n 1 -I _ echo _` : print the absolute paths of the changed files
- `fswatch --event Updated|Removed|Created|... path/to/directory | xargs -n 1 bash_command` : filter by event type

## Categories
files, monitoring

## Links
- More information: https://emcrisostomo.github.io/fswatch/
- tldr page: https://tldr.inbrowser.app/pages/common/fswatch
