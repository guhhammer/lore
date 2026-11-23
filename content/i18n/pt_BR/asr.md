# asr

Restaurar (copiar) uma imagem de disco em um volume. O nome do comando significa Apple Software Restore.

## Usage
- `sudo asr restore --source nome_da_imagem.dmg --target caminho/para/volume` : restaura uma imagem de disco para um volume de destino
- `sudo asr restore --source nome_da_imagem.dmg --target caminho/para/volume --erase` : apaga o volume de destino antes de restaurar
- `sudo asr restore --source nome_da_imagem.dmg --target caminho/para/volume --noverify` : ignora a verificação após a restauração
- `sudo asr restore --source caminho/para/volume --target caminho/para/volume_clonado` : clona volumes sem o uso de uma imagem de disco intermediária

## Links
- More information: https://keith.github.io/xcode-man-pages/asr.8.html
