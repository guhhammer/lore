# pacman-key

Script envoltório para o GnuPG usado para gerenciar o chaveiro do pacman.

## Usage
- `sudo pacman-key --init` : inicializa o chaveiro do `pacman`
- `sudo pacman-key --populate` : adiciona as chaves padrão do Arch Linux
- `pacman-key -l` : lista chaves do chaveiro público
- `sudo pacman-key -a caminho/para/arquivo_chave.gpg` : adiciona as chaves especificadas
- `sudo pacman-key -r "uid|nome|email"` : recebe uma chave do servidor de chaves
- `pacman-key -f "uid|nome|email"` : imprime a impressão digital de uma chave específica
- `sudo pacman-key --lsign-key "uid|nome|email"` : assina uma chave importada localmente
- `sudo pacman-key -d "uid|nome|email"` : remove uma chave específica

## Links
- More information: https://manned.org/pacman-key
