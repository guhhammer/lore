# gprof

Performance analysis tool for many programming languages. It profiles the function executions of a program.

## Usage
- `gcc -p program.c && ./a.out` : compile binary to default `a.out` with gprof information and run it to get `gmon.out`
- `gprof` : run gprof on default `a.out` and `gmon.out` to obtain profile output
- `gprof path/to/binary path/to/gmon.out` : run gprof on a named binary
- `gprof -b` : suppress profile field's description
- `gprof -bz` : display routines that have zero usage

## Categories
development, monitoring, network

## Links
- More information: https://ftp.gnu.org/old-gnu/Manuals/gprof/html_mono/gprof.html
- tldr page: https://tldr.inbrowser.app/pages/common/gprof
