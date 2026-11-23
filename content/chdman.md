# chdman

Manage and convert CHD (Compressed Hunks of Data) images. Commonly used with MAME and retro game images.

## Usage
- `chdman createcd -i path/to/file.cue -o path/to/file.chd` : create a CHD from a BIN/CUE pair (CD-ROM image)
- `chdman createhd -i path/to/disk.img -o path/to/disk.chd` : create a CHD from a raw hard drive image
- `chdman extractcd -i path/to/file.chd -o path/to/file.cue` : extract (decompress) a CHD back to BIN/CUE
- `chdman verify -i path/to/file.chd` : verify integrity of CHD file
- `chdman info -i path/to/file.chd` : view CHD metadata information
- `chdman copy -i path/to/old_file.chd -o path/to/new_file.chd` : update a CHD file to the latest format version
- `chdman extracthd -i path/to/disk.chd -o path/to/disk.img` : convert a compressed hard drive image to uncompressed (for editing)

## Categories
files, containers, media

## Links
- More information: https://docs.mamedev.org/tools/chdman.html
- tldr page: https://tldr.inbrowser.app/pages/common/chdman
