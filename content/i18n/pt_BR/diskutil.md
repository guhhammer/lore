# diskutil

Utilitário para gerenciar discos e volumes locais.

## Usage
- `diskutil list` : lista todos os discos, partições, e volumes montados atualmente disponíveis
- `diskutil repairVolume /dev/diskX` : repara as estruturas de dados do sistema de arquivos de um volume
- `diskutil unmountDisk /dev/diskX` : desmonta um volume
- `diskutil eject /dev/disk1` : ejeta um CD/DVD (desmonta primeiro)

## Links
- More information: https://keith.github.io/xcode-man-pages/diskutil.8.html
