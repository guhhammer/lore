# e2freefrag

Print the free space fragmentation information for ext2/ext3/ext4 filesystems.

## Usage
- `e2freefrag /dev/sdXN` : check how many free blocks are present as contiguous and aligned free space
- `e2freefrag -c chunk_size_in_kb /dev/sdXN` : specify chunk size in kilobytes to print how many free chunks are available

## Categories
documents

## Links
- More information: https://manned.org/e2freefrag
- tldr page: https://tldr.inbrowser.app/pages/linux/e2freefrag
