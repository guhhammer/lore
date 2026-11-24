# libtool

A generic library support script that hides the complexity of using shared libraries behind a consistent, portable interface.

## Usage
- `libtool c gcc -c path/to/source.c -o path/to/source.lo` : compile a source file into a `libtool` object
- `libtool l gcc -o path/to/library.lo path/to/source.lo` : create a library or an executable
- `libtool e gdb path/to/program` : automatically set the library path so that another program can use uninstalled `libtool` generated programs or libraries
- `libtool i cp path/to/library.la path/to/installation_directory` : install a shared library
- `libtool f path/to/installation_directory` : complete the installation of `libtool` libraries on the system
- `libtool u path/to/installed_library.la` : delete installed libraries or executables
- `libtool cl rm path/to/source.lo path/to/library.la` : delete uninstalled libraries or executables

## Categories
development, files, network

## Links
- More information: https://www.gnu.org/software/libtool/manual/libtool.html#Invoking-libtool
- tldr page: https://tldr.inbrowser.app/pages/linux/libtool
- Wikipedia: https://en.wikipedia.org/wiki/GNU_Libtool
