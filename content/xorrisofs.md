# xorrisofs

Emulation of ISO 9660 program mkisofs by program xorriso.

## Usage
- `xorrisofs [ options ] [-o filename ] pathspec [pathspecs ...]` : the general form, as the manual gives it
- `xorrisofs -prev-session disk_path` : alias of -M
- `xorrisofs -dev disk_path` : alias of -M
- `xorrisofs -follow-links` : alias of -f
- `xorrisofs -m disk_pattern` : exclude files from being inserted into the image

## Categories
shell
