# autoconf

Generate configuration scripts to automatically configure software source code packages.

## Usage
- `autoconf` : generate a configuration script from `configure.ac` (if present) or `configure.in` and save this script to `configure`
- `autoconf template_file` : generate a configuration script from the specified template; output to `stdout`
- `autoconf -f -o outfile template_file` : generate a configuration script from the specified template (even if the input file has not changed) and write the output to a file

## Categories
development, packages

## Links
- More information: https://manned.org/autoconf
- tldr page: https://tldr.inbrowser.app/pages/common/autoconf
- Wikipedia: https://en.wikipedia.org/wiki/Autoconf
