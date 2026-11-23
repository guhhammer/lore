# zmv

Move or rename files matching a specified extended `glob` pattern.

## Usage
- `zmv '(*).log' '$1.txt'` : move files using a `regex`-like pattern
- `zmv -n '(*).log' '$1.txt'` : preview the result of a move, without making any actual changes
- `zmv -i '(*).log' '$1.txt'` : interactively move files, with a prompt before every change
- `zmv -v '(*).log' '$1.txt'` : verbosely print each action as it's being executed

## Categories
files, text, monitoring

## Links
- More information: https://zsh.sourceforge.io/Doc/Release/User-Contributions.html
- tldr page: https://tldr.inbrowser.app/pages/common/zmv
