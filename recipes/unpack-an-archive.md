# unpack an archive

Open any common archive: tar, zip, gz, xz, 7z, rar.

## Steps
- `tar -xf archive.tar.gz` : any tar: .tar, .tar.gz, .tgz, .tar.xz, .tar.zst
- `tar -xf archive.tar.gz -C /dest` : into another folder
- `unzip archive.zip -d folder` : a zip, into a folder
- `gunzip file.gz` : a single .gz file
- `xz -d file.xz` : a single .xz file
- `7z x archive.7z` : 7z, and most other kinds (p7zip)
- `unrar x archive.rar` : rar, if unrar is installed

## Notes
- `tar -tf` and `unzip -l` list what is inside first.

## See also
- tar
- unzip
- 7z
- gunzip

## Categories
archives files
