# unsquashfs

Dekompresuj, rozpakuj i wyświetl listę plików w systemach plików squashfs.

## Usage
- `unsquashfs system_plików.squashfs` : rozpakuj system plików squashfs do `squashfs-root` w aktualnym katalogu roboczym
- `unsquashfs -d ścieżka/do/katalogu system_plików.squashfs` : rozpakuj system plików squashfs do podanego katalogu
- `unsquashfs -i system_plików.squashfs` : wyświetlaj nazwy plików podczas ich rozpakowywania
- `unsquashfs -li system_plików.squashfs` : wyświetlaj nazwy plików i ich atrybuty podczas ich rozpakowywania
- `unsquashfs -l system_plików.squashfs` : wyświetl listę plików w systemie plików squashfs (bez rozpakowywania)
- `unsquashfs -ll system_plików.squashfs` : wyświetl listę plików i ich atrybuty w systemie plików squashfs (bez rozpakowywania)

## Links
- More information: https://manned.org/unsquashfs
