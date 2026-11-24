# rpcgen

An RPC protocol compiler.

## Usage
- `rpcgen infile` : the general form, as the manual gives it
- `rpcgen [-Dname[=value]] [-T] [-K secs] infile` : another form of the same command
- `rpcgen -c|-h|-l|-m|-M|-t [-o outfile ] infile` : another form of the same command
- `rpcgen [-I] -s nettype [-o outfile] infile` : another form of the same command
- `rpcgen -n netid [-o outfile] infile` : another form of the same command
- `rpcgen -b` : this generates code for the SunOS4.1 style of rpc
- `rpcgen -c` : compile into XDR routines

## Categories
development

## Links
- Wikipedia: https://en.wikipedia.org/wiki/RPCGEN
