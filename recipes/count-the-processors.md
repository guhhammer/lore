# count the processors

How many processors and cores this machine has, and what they are.

## Steps
- `nproc` : the processors programs can use
- `lscpu` : the model, cores, threads and caches
- `grep -c ^processor /proc/cpuinfo` : the same count, from the kernel
- `sysctl -n hw.ncpu` : macOS

## See also
- nproc
- lscpu
- sysctl

## Categories
system hardware
