# htcacheclean

Clean up the disk cache.

## Usage
- `htcacheclean [ -D ] [ -v ] [ -t ] [ -r ] [ -n ] [ -Rround ] -ppath [ -llimit ] [ -Llimit ]` : the general form, as the manual gives it
- `htcacheclean [ -n ] [ -t ] [ -i ] [ -Ppidfile ] [ -Rround ] -dinterval -ppath [ -llimit ] [ -Llimit ]` : another form of the same command
- `htcacheclean [ -v ] [ -Rround ] -ppath [ -a ] [ -A ]` : another form of the same command
- `htcacheclean [ -D ] [ -v ] [ -t ] [ -Rround ] -ppath url` : another form of the same command
- `htcacheclean -dinterval` : daemonize and repeat cache cleaning every interval minutes
- `htcacheclean -D` : do  a dry run and don't delete anything
- `htcacheclean -v` : be verbose and print statistics

## Categories
disks
