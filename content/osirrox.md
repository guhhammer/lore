# osirrox

Creates, loads, manipulates and writes ISO 9660 filesystem images with Rock Ridge extensions.

## Usage
- `xorriso [settings|actions]` : the general form, as the manual gives it
- `osirrox -indev address` : set  input drive and load an ISO image if present
- `osirrox -read_speed code|number[k|m|c|d|b]` : set the speed for reading
- `osirrox -ban_stdio_write` : allow for writing only the usage of MMC optical drives
- `osirrox -quoted_path_list disk_path` : like -path_list but with quoted input reading rules

## Categories
containers, disks, media
