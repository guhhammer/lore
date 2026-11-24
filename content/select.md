# select

Bash builtin construct for creating menus.

## Usage
- `select word in apple orange pear banana; do echo $word; done` : create a menu out of individual words
- `select line in $(command); do echo $line; done` : create a menu from the output of another command
- `PS3="Select a file: "; select file in *; do echo $file; done` : specify the prompt string for `select` and create a menu for picking a file or folder from the current directory
- `fruits=(apple orange pear banana); select word in ${fruits[@]}; do echo $word; done` : create a menu from a Bash array

## Categories
text, files, shell

## Links
- More information: https://www.gnu.org/software/bash/manual/bash.html#index-select
- tldr page: https://tldr.inbrowser.app/pages/common/select
- Wikipedia: https://en.wikipedia.org/wiki/Select_(Unix)
