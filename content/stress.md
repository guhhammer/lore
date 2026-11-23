# stress

Stress test CPU, memory, and IO on a Linux system.

## Usage
- `stress -c 4` : spawn 4 workers to stress test CPU
- `stress -i 2 -t 5` : spawn 2 workers to stress test IO and timeout after 5 seconds
- `stress -m 2 --vm-bytes 256M` : spawn 2 workers to stress test memory (each worker allocates 256M bytes)
- `stress -d 2 --hdd-bytes 1GB` : spawn 2 workers spinning on write()/unlink() (each worker writes 1G bytes)

## Categories
development, hardware, monitoring

## Links
- More information: https://manned.org/stress
- tldr page: https://tldr.inbrowser.app/pages/linux/stress
