# 7za

Archiviatore di file con alto rapporto di compressione. Simile a `7z` ma supporta meno tipi di file ed è multipiattaforma.

## Usage
- `7za a percorso/all/archivio.7z percorso/del/file_o_directory` : archivia un file o directory
- `7za a percorso/all/archivio_crittografato.7z -ppassword -mhe=on percorso/all/archivio.7z` : crittografa un archivio esistente (inclusi i nomi dei file)
- `7za x percorso/all/archivio.7z` : estrai un archivio preservando la struttura delle directory originali
- `7za x percorso/all/archivio.7z -opercorso/all/output` : estrai un archivio in una directory specifica
- `7za x percorso/all/archivio.7z -so` : estrai un archivio su `stdout`
- `7za a -t7z|bzip2|gzip|lzip|tar|... percorso/all/archivio.7z percorso/del/file_o_directory` : archivia usando un tipo di archivio specifico
- `7za l percorso/all/archivio.7z` : lista i contenuti di un archivio
- `7za a percorso/all/archivio.7z -mx=0|1|3|5|7|9 percorso/del/file_o_directory` : imposta il livello di compressione (più alto = maggiore compressione ma più lento)

## Links
- More information: https://manned.org/7za
