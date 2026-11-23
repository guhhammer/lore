# zipsplit

Split a Zip archive into smaller Zip archives.

## Usage
- `zipsplit path/to/archive.zip` : split Zip archive into parts that are no larger than 36000 bytes (36 MB)
- `zipsplit -n size path/to/archive.zip` : use a given number of bytes as the part limit
- `zipsplit -p -n size path/to/archive.zip` : pause between the creation of each part
- `zipsplit -b path/to/output_directory -n size path/to/archive.zip` : output the smaller Zip archives into a given directory

## Categories
archives, files

## Links
- More information: https://manned.org/zipsplit
- tldr page: https://tldr.inbrowser.app/pages/linux/zipsplit
