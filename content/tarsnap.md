# tarsnap

Manipulate remote Tarsnap encrypted backups. Note: You don't need to specify the key file and the cache directory if you configure them in `/usr/local/etc/tarsnap.conf` or `~/.tarsnaprc`.

## Usage
- `tarsnap -c --keyfile path/to/key_file --cachedir path/to/cache_directory -f archive_name path/to/file_or_directory1 path/to/file_or_directory2 ...` : create a backup archive of one or more files or directories, specifying the cryptographic key and the cache directory
- `tarsnap -c --dry-run --print-stats --keyfile path/to/key_file --cachedir path/to/cache_directory -f archive_name path/to/file_or_directory1 path/to/file_or_directory2 ...` : simulate creating an archive, displaying how much data would be uploaded
- `tarsnap --list-archives --keyfile path/to/key_file` : list stored archives
- `tarsnap -d --keyfile path/to/key_file --cachedir path/to/cache_directory -f archive_name` : delete a specific archive
- `tarsnap -tv --keyfile path/to/key_file -f archive_name` : list the contents of a specific archive in verbose mode
- `tarsnap -x --keyfile path/to/key_file -f archive_name path/to/file_or_directory1 path/to/file_or_directory2 ...` : restore one or more files or directories from a specific archive
- `tarsnap -c --keyfile path/to/key_file -f new_archive_name @@source_archive_name` : copy an archive

## See also
- tarsnap-keygen

## Categories
files, archives

## Links
- More information: https://www.tarsnap.com/man-tarsnap.1.html
- tldr page: https://tldr.inbrowser.app/pages/common/tarsnap
