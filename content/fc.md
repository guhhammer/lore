# fc

Open the recent commands for editing and then run them.

## Usage
- `fc` : open the last command in the default system editor and run it after editing
- `fc -e 'emacs'` : specify an editor to open with
- `fc -l` : list recent commands from history
- `fc -l -r` : list recent commands in reverse order
- `fc number` : edit and run a command from history
- `fc '416' '420'` : edit commands in a given interval and run them
- `fc -s string1=string2` : run the previous command immediately without opening an editor and replace all occurrences of a string with another string
- `fc --help` : display help

## See also
- history

## Categories
editors, shell

## Links
- More information: https://www.gnu.org/software/bash/manual/bash.html#index-fc
- tldr page: https://tldr.inbrowser.app/pages/common/fc
