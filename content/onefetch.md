# onefetch

Display project information and code statistics for a local Git repository.

## Usage
- `onefetch` : display statistics for the Git repository in the current working directory
- `onefetch path/to/directory` : display statistics for the Git repository in the specified directory
- `onefetch --no-bots` : ignore commits made by bots
- `onefetch --no-merges` : ignore merge commits
- `onefetch --no-art` : don't print the ASCII art of the language logo
- `onefetch --number-of-authors|languages|file-churns n` : show `n` authors, languages, or file churns (default: 3, 6, and 3 respectively)
- `onefetch -e path/to/file_or_directory|regex` : ignore the specified files and directories
- `onefetch -T programming|markup|prose|data` : only detect languages from the specified categories (default: programming and markup)

## Categories
files, version-control, desktop

## Links
- More information: https://github.com/o2sh/onefetch/wiki/command-line-options
- tldr page: https://tldr.inbrowser.app/pages/common/onefetch
