# mksquashfs

Utwórz lub dodaj pliki i katalogi do systemów plików squashfs.

## Usage
- `mksquashfs ścieżka/do/pliku_lub_katalogu1 ścieżka/do/pliku_lub_katalogu2 ... system_plików.squashfs` : utwórz lub dodaj pliki i katalogi do systemu plików squashfs (domyślnie kompresując za pomocą `gzip`)
- `mksquashfs ścieżka/do/pliku_lub_katalogu1 ścieżka/do/pliku_lub_katalogu2 ... system_plików.squashfs -comp gzip|lzo|lz4|xz|zstd|lzma` : utwórz lub dodaj pliki i katalogi do systemu plików squashfs, używając podanego algorytmu kompresji ([comp]ression)
- `mksquashfs ścieżka/do/pliku_lub_katalogu1 ścieżka/do/pliku_lub_katalogu2 ... system_plików.squashfs -e plik|katalog1 plik|katalog2 ...` : utwórz lub dodaj pliki i katalogi do systemu plików squashfs, pomijając (excluding) niektóre z nich
- `mksquashfs ścieżka/do/pliku_lub_katalogu1 ścieżka/do/pliku_lub_katalogu2 ... system_plików.squashfs -wildcards -e "*.gz"` : utwórz lub dodaj pliki i katalogi do systemu plików squashfs, pomijając (excluding) te kończące się na `.gz`
- `mksquashfs ścieżka/do/pliku_lub_katalogu1 ścieżka/do/pliku_lub_katalogu2 ... system_plików.squashfs -regex -e "wyrażenie_regularne"` : utwórz lub dodaj pliki i katalogi do systemu plików squashfs, pomijając (excluding) te pasujące do wyrażenia regularnego

## Links
- More information: https://manned.org/mksquashfs
