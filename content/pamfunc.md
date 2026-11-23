# pamfunc

Apply a simple arithmetic function to a Netpbm image.

## Usage
- `pamfunc -multiplier|divisor|adder|subtractor|min|max n path/to/input.pam >path/to/output.pam` : apply the specified arithmetic function with `n` as the second argument to each sample in the specified PAM image
- `pamfunc -andmask|ormask|xormask|shiftleft|shiftright n path/to/input.pam >path/to/output.pam` : apply the specified bit string function with `n` as the second argument to each sample in the specified PAM image

## Categories
containers, media, files

## Links
- More information: https://netpbm.sourceforge.net/doc/pamfunc.html
- tldr page: https://tldr.inbrowser.app/pages/common/pamfunc
