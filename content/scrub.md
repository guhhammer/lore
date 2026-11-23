# scrub

Securely overwrite files, block devices, or free disk space.

## Usage
- `scrub path/to/file` : securely overwrite a file using the default overwrite pattern
- `scrub path/to/file1 path/to/file2 ...` : securely overwrite multiple files
- `sudo scrub /dev/sdX` : securely overwrite a block device or partition
- `scrub -p random path/to/file` : securely overwrite a file with a single-pass overwrite pattern
- `scrub -p dod path/to/file` : securely overwrite a file using the DoD 5220.22-M overwrite pattern
- `scrub -p gutmann path/to/file` : securely overwrite a file using the Gutmann overwrite pattern
- `scrub -r path/to/file` : remove a file after successfully overwriting it
- `scrub -X path/to/directory` : securely overwrite free space in a directory

## Categories
files, disks, hardware

## Links
- More information: https://manned.org/scrub
- tldr page: https://tldr.inbrowser.app/pages/linux/scrub
