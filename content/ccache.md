# ccache

C/C++ compiler cache. Note: Packages usually provide symlinks for compilers in `/usr/lib/ccache/bin`. Prepend this directory to `$PATH` to automatically use `ccache` for them.

## Usage
- `ccache -s` : show current cache statistics
- `ccache -C` : clear all cache
- `ccache -z` : reset statistics (but not cache itself)
- `ccache gcc path/to/file.c` : compile C code and cache compiled output (to use `ccache` on all `gcc` invocations, see the note above)

## Categories
files, development, packages

## Links
- More information: https://ccache.dev/manual/latest.html
- tldr page: https://tldr.inbrowser.app/pages/common/ccache
- Wikipedia: https://en.wikipedia.org/wiki/Ccache
