# lsblk

Lista informações sobre dispositivos.

## Usage
- `lsblk` : lista todos dispositivos de armazenamento no formato de árvore
- `lsblk -a` : também lista dispositivos vazios
- `lsblk -b` : mostra a coluna de tamanhos em bytes, em vez de um formato legível por humanos
- `lsblk -f` : mostra na saída padrão informações sobre os filesystems dos dispositivos
- `lsblk -i` : utiliza caracteres ASCII para o formato de árvore
- `lsblk -t` : mostra na saída padrão informações sobre block-device topology
- `lsblk -e 1,7,...` : excluír da saída padrão os dispositivos específicados por seus repectivos números separados por vírgulas
- `lsblk -o NAME,SERIAL,MODEL,TRAN,TYPE,SIZE,FSTYPE,MOUNTPOINT,...` : mostra um resumo de forma customizada passando as colunas separadas por vírgulas

## Links
- More information: https://manned.org/lsblk
