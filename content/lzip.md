# lzip

A lossless data compressor with a user interface similar to `gzip` or `bzip2`. Lzip uses a simplified form of the "Lempel-Ziv-Markovchain-Algorithm" (LZMA) stream format and provides 3-factor integrity checking to maximize interoperability and optimize safety.

## Usage
- `lzip path/to/file` : archive a file, replacing it with a compressed version
- `lzip -k path/to/file` : archive a file, keeping the input file
- `lzip -k path/to/file --best` : archive a file with the best compression (level=9)
- `lzip -k path/to/file --fast` : archive a file at the fastest speed (level=0)
- `lzip -t path/to/archive.lz` : test the integrity of compressed file
- `lzip -d path/to/archive.lz` : decompress a file, replacing it with the original uncompressed version
- `lzip -d -k path/to/archive.lz` : decompress a file, keeping the archive
- `lzip -l path/to/archive.lz` : list files which are in an archive and show compression stats

## Categories
files, archives, network

## Links
- More information: https://www.nongnu.org/lzip/manual/lzip_manual.html#Invoking-lzip
- tldr page: https://tldr.inbrowser.app/pages/common/lzip
