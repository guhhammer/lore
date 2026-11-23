# popd

Remove a directory placed on the directory stack via the `pushd` shell built-in.

## Usage
- `popd` : remove the top directory from the stack and cd to it
- `popd +N` : remove the Nth directory (starting from zero to the left from the list printed with `dirs`)
- `popd -N` : remove the Nth directory (starting from zero to the right from the list printed with `dirs`)
- `popd -n` : remove the 1st directory (starting from zero to the left from the list printed with `dirs`)

## See also
- pushd
- dirs

## Categories
files, shell

## Links
- More information: https://www.gnu.org/software/bash/manual/bash.html#index-popd
- tldr page: https://tldr.inbrowser.app/pages/common/popd
