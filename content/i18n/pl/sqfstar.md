# sqfstar

Utwórz system plików squashfs z archiwum `.tar`.

## Usage
- `sqfstar <archiwum.tar system_plików.squashfs` : utwórz system plików squashfs (domyślnie kompresując za pomocą `gzip`) z nieskompresowanego archiwum `.tar`
- `zcat archiwum.tar.gz | sqfstar -comp gzip|lzo|lz4|xz|zstd|lzma system_plików.squashfs` : utwórz system plików squashfs z archiwum `.tar` skompresowanego za pomocą `gzip`, i skompresuj system plików używając podanego algorytmu
- `xzcat archiwum.tar.xz | sqfstar system_plików.squashfs plik1 plik2 ...` : utwórz system plików squashfs z archiwum `.tar` skompresowanego za pomocą `xz`, pomijając niektóre pliki
- `zstdcat archiwum.tar.zst | sqfstar system_plików.squashfs "*.gz"` : utwórz system plików squashfs z archiwum `.tar` skompresowanego za pomocą `zstd`, pomijając pliki kończące się na `.gz`
- `lz4cat archiwum.tar.lz4 | sqfstar system_plików.squashfs -regex "wyrażenie_regularne"` : utwórz system plików squashfs z archiwum `.tar` skompresowanego za pomocą `lz4`, pomijając pliki pasujące do wyrażenia regularnego

## Links
- More information: https://manned.org/sqfstar
