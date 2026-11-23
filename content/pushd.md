# pushd

Place a directory on a stack so it can be accessed later.

## Usage
- `pushd path/to/directory` : switch to directory and push it on the stack
- `pushd` : switch first and second directories on the stack
- `pushd +4` : rotate stack by making the 5th element the top of the stack
- `pushd -n +4` : rotate the stack 4 times to the left (the current directory stays at the top by replacing the 5th element)

## See also
- popd
- dirs

## Categories
files, shell

## Links
- More information: https://www.gnu.org/software/bash/manual/bash.html#index-pushd
- tldr page: https://tldr.inbrowser.app/pages/common/pushd
