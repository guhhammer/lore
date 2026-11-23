# vmtouch

Manage the filesystem cache.

## Usage
- `vmtouch path/to/file` : print the cache status of a file
- `vmtouch -t path/to/file` : load a file into cache
- `vmtouch -e path/to/file` : evict a file from cache
- `vmtouch -l path/to/file` : lock a file in cache to prevent eviction from memory
- `vmtouch -ld path/to/file` : lock a file and daemonize the program

## Categories
files, disks

## Links
- More information: https://manned.org/vmtouch
- tldr page: https://tldr.inbrowser.app/pages/common/vmtouch
