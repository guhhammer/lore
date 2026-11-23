# vipe

Run a text editor in the middle of a UNIX pipeline.

## Usage
- `command1 | vipe | command2` : edit the output of `command1` before piping it into `command2`
- `command1 | vipe --suffix json | command2` : buffer the output of `command1` in a temporary file with the specified file extension in order to aid syntax highlighting
- `command1 | EDITOR=vim vipe | command2` : use the specified text editor

## Categories
editors, text

## Links
- More information: https://manned.org/vipe
- tldr page: https://tldr.inbrowser.app/pages/common/vipe
