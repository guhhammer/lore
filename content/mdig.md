# mdig

DNS pipelined lookup utility.

## Usage
- `mdig {-h}` : the general form, as the manual gives it
- `mdig [@server] {global-opt...} { {local-opt...} {query} ...}` : another form of the same command
- `mdig -v` : this option causes mdig to print the version number and exit
- `mdig -4` : this option forces mdig to only use IPv4 query transport
- `mdig -6` : this option forces mdig to only use IPv6 query transport
- `mdig -b address` : this option sets the source IP address of the query to address

## Categories
network, databases

## Links
- Documentation: https://datatracker.ietf.org/doc/html/rfc3597
