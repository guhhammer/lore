# sc_tracediff

Display traceroute paths where the path has changed.

## Usage
- `sc_tracediff path/to/file1.warts path/to/file2.warts` : show the difference between traceroutes in two `.warts` files
- `sc_tracediff -a path/to/file1.warts path/to/file2.warts` : show the difference between the traceroutes in two `.warts` files, including those that have not changed
- `sc_tracediff -n path/to/file1.warts path/to/file2.warts` : show the difference between the traceroutes in two `.warts` files and try to show DNS names and not IP addresses if possible

## Categories
files, desktop, network

## Links
- More information: https://www.caida.org/catalog/software/scamper/
- tldr page: https://tldr.inbrowser.app/pages/common/sc_tracediff
