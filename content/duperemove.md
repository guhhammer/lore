# duperemove

Find duplicate filesystem extents and optionally schedule them for deduplication. An extent is small part of a file inside the filesystem. On some filesystems one extent can be referenced multiple times, when parts of the content of the files are identical.

## Usage
- `duperemove -r path/to/directory` : search for duplicate extents in a directory and show them
- `duperemove -r -d path/to/directory` : deduplicate duplicate extents on a Btrfs or XFS (experimental) filesystem
- `duperemove -r -d --hashfile=path/to/hashfile path/to/directory` : use a hash file to store extent hashes (less memory usage and can be reused on subsequent runs)
- `duperemove -r -d --hashfile=path/to/hashfile --io-threads=n --cpu-threads=n path/to/directory` : limit I/O threads (for hashing and dedupe stage) and CPU threads (for duplicate extent finding stage)

## Categories
files, disks, system

## Links
- More information: https://markfasheh.github.io/duperemove/
- tldr page: https://tldr.inbrowser.app/pages/linux/duperemove
