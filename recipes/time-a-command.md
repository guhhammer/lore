# time a command

How long a command takes, and how much memory and processor it used.

## Steps
- `time command` : real is the clock time; user and sys, the processor's
- `/usr/bin/time -v command` : also its peak memory (GNU time)
- `hyperfine 'command'` : run it many times for a fair figure (hyperfine)

## Notes
- real much bigger than user + sys means it was waiting: the disk, the network or you.

## See also
- time
- hyperfine

## Categories
development processes
