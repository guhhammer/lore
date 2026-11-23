# fsck

Verifica a integridade de um sistema de arquivos ou repara ele. O sistema de arquivos deve ser desmontado no momento em que o comando é executado. É um wrapper que chama `fsck_hfs`, `fsck_apfs`, `fsck_msdos`, `fsck_exfat`, e `fsck_udf` conforme necessário.

## Usage
- `fsck /dev/sdX` : verifica o sistema de arquivos `/dev/sdX`, relatando quaisquer blocos danificados
- `fsck -f /dev/sdX` : verifica o sistema de arquivos `/dev/sdX` apenas se estiver limpo, relatando quaisquer blocos danificados e permitindo que o usuário interativamente escolha reparar cada um deles
- `fsck -fy /dev/sdX` : verifica o sistema de arquivos `/dev/sdX` apenas se estiver limpo, relatando quaisquer blocos danificados e reparando-os automaticamente
- `fsck -q /dev/sdX` : verifica o sistema de arquivos `/dev/sdX`, informando se ele foi desmontado corretamente

## Links
- More information: https://keith.github.io/xcode-man-pages/fsck.8.html
