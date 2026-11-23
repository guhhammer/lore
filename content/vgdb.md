# vgdb

Intermediary between Valgrind and GDB or a shell.

## Usage
- `vgdb [options]` : the general form, as the manual gives it
- `vgdb --pid=<number>` : specifies the PID of the process to which vgdb must connect to
- `vgdb --valgrind` : the path to valgrind to use, in extended-remote mode
- `vgdb --vargs` : options to run valgrind with, in extended-remote mode
- `vgdb -T` : instructs vgdb to add timestamps to vgdb information messages

## Categories
processes, shell
