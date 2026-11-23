# distcc

Distributed C/C++/ObjC compilation client that works with `distccd`.

## Usage
- `distcc gcc -c path/to/source.c -o path/to/output.o` : compile a source file using a compiler like `gcc`
- `export DISTCC_HOSTS="localhost ip1 ip2 ..."` : set remote hosts to distribute compilation
- `make -j parallel_jobs CC="distcc gcc"` : compile a project with `make` using `distcc`
- `distcc --show-hosts` : show the list of current `distcc` hosts
- `distcc --help` : display help
- `distcc --version` : display version

## Categories
development, files

## Links
- More information: https://manned.org/distcc
- tldr page: https://tldr.inbrowser.app/pages/common/distcc
