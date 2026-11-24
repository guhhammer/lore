# getopts

Parse shell options from arguments. Note: This command does not support longform options and thus using `getopt` is recommended instead.

## Usage
- `getopts x opt; echo $opt` : check if an option is the first set option in the current context
- `getopts x opt "string text"; echo $opt` : check if an option is set in a string (specified option must be the first element of the string)
- `getopts x: opt; echo $opt $OPTARG` : set an option to require an argument and print them
- `while getopts xyz opt; do case $opt in x) echo x is set;; y) echo y is set;; z) echo z is set;; esac; done` : check for multiple options
- `while getopts :x: opt; do case $opt in x) ;; :) echo "Argument required";; ?) echo "Invalid argument" esac;; done` : set `getopts` to silent mode and handle option errors
- `OPTIND=1` : reset `getopts`

## Categories
text, shell

## Links
- More information: https://www.gnu.org/software/bash/manual/bash.html#index-getopts
- tldr page: https://tldr.inbrowser.app/pages/common/getopts
- Wikipedia: https://en.wikipedia.org/wiki/Getopts
