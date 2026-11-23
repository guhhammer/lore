# zstyle

Define and look up configuration styles in Zsh. This builtin is part of the `zsh/zutil` module.

## Usage
- `zstyle` : list all defined styles
- `zstyle -L` : list styles as ready-to-paste `zstyle` commands
- `zstyle pattern style value1 value2...` : define a style for a specific pattern
- `zstyle -d pattern` : delete a style for a specific pattern
- `zstyle -s context style variable_name` : retrieve a style value as a string into a variable
- `zstyle -b context style variable_name` : retrieve a style value as a boolean into a variable
- `zstyle -a context style variable_name` : retrieve a style value as an array into a variable

## Categories
text, development, shell

## Links
- More information: https://zsh.sourceforge.io/Doc/Release/Zsh-Modules.html
- tldr page: https://tldr.inbrowser.app/pages/common/zstyle
