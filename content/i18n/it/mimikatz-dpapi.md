# mimikatz dpapi

Interagisce con la Windows Data Protection API (DPAPI).

## Usage
- `mimikatz "dpapi::masterkey /list"` : elenca le chiavi master
- `mimikatz "dpapi::blob /in:blob_file.bin"` : decripta un blob DPAPI
- `mimikatz "dpapi::chrome /in:Login Data"` : recupera le credenziali di Chrome usando DPAPI

## Links
- More information: https://github.com/gentilkiwi/mimikatz
