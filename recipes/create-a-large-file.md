# create a large file

Make a file of a given size, for testing disks, uploads or swap.

## Steps
- `fallocate -l 1G big.img` : 1 GB at once, reserved on the disk (Linux)
- `truncate -s 1G sparse.img` : 1 GB that takes no room until written
- `dd if=/dev/zero of=big.img bs=1M count=1024 status=progress` : 1 GB of zeros, written out
- `head -c 100M /dev/urandom > random.bin` : 100 MB of random bytes

## See also
- fallocate
- truncate
- dd

## Categories
files disks
