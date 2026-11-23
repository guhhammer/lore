# do

This shell keyword is used with `for`, `while`, `select`, and `until` to mark the start of a loop.

## Usage
- `for variable; do echo $variable; done` : iterate through command line parameters
- `for variable in item1 item2 ...; do echo "Loop is executed"; done` : execute the given commands for each of the specified items
- `while read line; do echo "$line"; done` : read `stdin` and perform an action on every line
- `while :; do command; sleep 1; done` : execute a command forever once every second
- `select word in apple orange pear banana; do echo $word; done` : create a menu out of individual words
- `select line in $(command); do echo $line; done` : create a menu from the output of another command
- `until command; do :; done` : execute a command until it succeeds
- `until ssh username@host; do sleep 2; done` : try to connect to an SSH host until gracefully disconnected

## Categories
text, network, shell

## Links
- tldr page: https://tldr.inbrowser.app/pages/common/do
