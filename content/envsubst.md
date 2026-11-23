# envsubst

Substitute environment variables with their value in shell format strings. Variables to be replaced should be in either `${var}` or `$var` format.

## Usage
- `echo '$HOME' | envsubst` : replace environment variables in `stdin` and output to `stdout`
- `envsubst <path/to/input_file` : replace environment variables in an input file and output to `stdout`
- `envsubst <path/to/input_file >path/to/output_file` : replace environment variables in an input file and output to a file
- `envsubst <path/to/input_file '$USER $SHELL $HOME'` : replace environment variables in an input file from a space-separated list

## Categories
files, shell

## Links
- More information: https://www.gnu.org/software/gettext/manual/gettext.html#envsubst-Invocation
- tldr page: https://tldr.inbrowser.app/pages/common/envsubst
