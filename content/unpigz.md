# unpigz

Compress or expand files.

## Usage
- `pigz [ -cdfhikKlLmMnNqrRtz0..9,11 ] [ -b blocksize ] [ -p threads ] [ -S suffix ] [ name ... ]` : the general form, as the manual gives it
- `unpigz [ -cfhikKlLmMnNqrRtz ] [ -b blocksize ] [ -p threads ] [ -S suffix ] [ name ... ]` : another form of the same command
- `unpigz -I` : number of iterations for optimization (default 15)
- `unpigz -J` : maximum number of split blocks (default 15)

## Categories
archives
