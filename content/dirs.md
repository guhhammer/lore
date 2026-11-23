# dirs

Display or manipulate the directory stack. The directory stack is a list of recently visited directories that can be manipulated with the `pushd` and `popd` commands.

## Usage
- `dirs` : display the directory stack with a space between each entry
- `dirs -p` : display the directory stack with one entry per line
- `dirs -v` : display a numbered list of entries in the directory stack
- `dirs -l` : display the directory stack without the tilde-prefix (`~`)
- `dirs +n` : display only the `n`th entry in the directory stack, starting at 0 (Bash only)
- `dirs -n` : display only the `n`th entry in the directory stack from the last, starting at 0 (Bash only)
- `dirs -c` : clear the directory stack

## See also
- pushd
- popd

## Categories
files, desktop, shell

## Links
- More information: https://www.gnu.org/software/bash/manual/bash.html#Directory-Stack-Builtins
- tldr page: https://tldr.inbrowser.app/pages/common/dirs
