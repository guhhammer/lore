# what is slowing the machine

See whether the processor, the memory or a disk is the bottleneck, and
which program is behind it.

## Steps
- `uptime` : the load over 1, 5 and 15 minutes; compare it with the number of processors (`nproc`)
- `top` : the busiest programs, live; on Linux, P sorts by processor and M by memory
- `ps aux --sort=-%cpu | head` : the 10 programs using the most processor (Linux)
- `ps aux --sort=-%mem | head` : the 10 using the most memory (Linux)
- `free -h` : memory and swap in use; a full swap slows everything
- `vmstat 1 5` : processor, memory, swap and disk waits, every second, five times
- `df -h` : a full disk slows programs that write to it

## Notes
- A load well above the number of processors means programs are waiting their turn.
- A high `wa` in `vmstat` or `top` means waiting for a disk: `iostat -x 1` (from sysstat) shows which one.
- macOS: `top -o cpu`, `top -o mem`, and `vm_stat` for memory.

## See also
- top
- htop
- ps
- vmstat
- free

## Categories
processes system
