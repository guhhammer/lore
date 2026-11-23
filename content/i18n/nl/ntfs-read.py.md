# ntfs-read.py

Een alleen-lezen NTFS-verkenner voor het openen en extraheren van bestanden van NTFS-volumes. Onderdeel van de Impacket-suite.

## Usage
- `ntfs-read.py volume` : open een NTFS-volume voor verkenning (bijvoorbeeld `C:\.\\` of `/dev/disk1s1`)
- `ntfs-read.py -extract \windows\system32\config\sam volume` : haal een specifiek bestand uit een NTFS-volume (bijvoorbeeld `\windows\system32\config\sam`)
- `ntfs-read.py -debug volume` : schakel debug-uitvoer in
- `ntfs-read.py --help` : toon de help

## Links
- More information: https://github.com/fortra/impacket
