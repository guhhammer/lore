# make a script take options

Read a script's arguments and options, and stop with a usage line when they are wrong.

## Steps
- `echo "$1 $2 ($# arguments)"` : the first two arguments, and how many there are
- `[ $# -eq 0 ] && { echo "usage: $0 FILE"; exit 1; }` : stop when none were given
- `for arg in "$@"; do echo "$arg"; done` : go through every argument, spaces kept
- `while getopts 'vf:' opt; do case $opt in v) verbose=1;; f) file=$OPTARG;; *) exit 1;; esac; done` : -v and -f FILE options
- `shift $((OPTIND - 1))` : after getopts: what is left are the plain arguments
- `name="${1:-world}"` : a default when no argument was given

## See also
- getopts
- shift

## Categories
shell development
