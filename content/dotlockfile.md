# dotlockfile

Utility to manage lockfiles.

## Usage
- `dotlockfile -l [-r retries] [-i interval] [-p] [-q] <-m | lockfile>` : the general form, as the manual gives it
- `dotlockfile -l [-r retries] [-i interval] [-p] [-q] <-m | lockfile> [-P] cmd args ...` : another form of the same command
- `dotlockfile -u | -t` : another form of the same command
- `dotlockfile -i interval` : sets a consistent retry interval
- `dotlockfile -u` : remove a lockfile
- `dotlockfile -q` : don't print warnings or errors to the standard error output
