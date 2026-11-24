# see a program's memory use

How much memory a program really uses, and which programs use the most.

## Steps
- `ps aux --sort=-rss | head` : the programs using the most memory, first (Linux)
- `ps -o pid,rss,vsz,cmd -p PID` : one program: RSS is the memory it really holds, in KB
- `grep -E 'VmRSS|VmSwap' /proc/PID/status` : its memory and its swap (Linux)
- `pmap -x PID | tail -1` : the total of every part of its memory
- `smem -k -P NAME` : memory shared counted fairly, by name (smem)
- `free -h` : the whole machine's memory

## Notes
- Put the program's number in place of PID: `pgrep -l NAME` finds it.
- macOS: `ps -o pid,rss,command -p PID` and Activity Monitor.

## See also
- ps
- pmap
- free
- top

## Categories
processes system
