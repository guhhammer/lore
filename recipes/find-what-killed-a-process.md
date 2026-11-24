# find what killed a process

A program vanished or says "Killed": usually the kernel ran out of memory and stopped it. Find out.

## Steps
- `sudo dmesg -T | grep -i -E 'killed process|out of memory'` : the kernel's out-of-memory kills, with times
- `journalctl -k | grep -i oom` : the same from the journal
- `journalctl -u NAME --since today` : a service's own last words
- `echo $?` : right after it ended: 137 means killed by force (128 + 9), 143 asked to stop
- `free -h` : how much memory and swap there is

## See also
- dmesg
- journalctl
- free

## Categories
processes system
