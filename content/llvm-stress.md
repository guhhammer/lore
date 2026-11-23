# llvm-stress

Generate random .ll files.

## Usage
- `llvm-stress [-size=filesize] [-seed=initialseed] [-o=outfile]` : the general form, as the manual gives it
- `llvm-stress -o filename` : specify the output filename
- `llvm-stress -size size` : specify the size of the generated .ll file
- `llvm-stress -seed seed` : specify the seed to be used for the randomly generated instructions

## Links
- Documentation: https://llvm.org/
