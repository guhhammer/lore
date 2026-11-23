# sdelete

Securely delete file/directory from disk, or clean the free space on a volume/physical disk.

## Usage
- `sdelete -p 3 path\to\file1 path\to\file2 ...` : delete files with 3 passes
- `sdelete -s path\to\directory1 path\to\directory2 ...` : delete folders and its subdirectories with 1 pass (default)
- `sdelete -p 3 D:` : clean the free space of volume D: with 3 passes
- `sdelete -z 2` : clean the free space with zeros of physical disk 2, which should not contain any volumes to be cleaned

## Notes
A Windows command: this page is for reference, it does not run on Linux or macOS.

## Categories
disks, files, windows

## Links
- More information: https://learn.microsoft.com/sysinternals/downloads/sdelete
- tldr page: https://tldr.inbrowser.app/pages/windows/sdelete
