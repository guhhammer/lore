# mkfs.fat

Cria um sistema de arquivos MS-DOS dentro de uma partição.

## Usage
- `mkfs.fat /dev/sdb1` : cria um sistema de arquivos fat dentro da partição 1 do dispositivo b (`sdb1`)
- `mkfs.fat -n nome_de_volume /dev/sdb1` : cria um sistema de arquivos com um nome de volume
- `mkfs.fat -i id_de_volume /dev/sdb1` : cria um sistema de arquivos com um ID de volume
- `mkfs.fat -f 5 /dev/sdb1` : usa 5 em vez de 2 tabelas de alocação de arquivos

## Links
- More information: https://manned.org/mkfs.fat
