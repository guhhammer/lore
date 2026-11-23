# makepkg

Monta um pacote que pode ser usado junto ao `pacman`. Utiliza por padrão o arquivo `PKGBUILD` no diretório de trabalho atual.

## Usage
- `makepkg` : monta um pacote
- `makepkg -s` : monta um pacote e instala suas dependências
- `makepkg -s -i` : monta um pacote, instala suas dependências e então o instala no sistema
- `makepkg --skipchecksums` : monta um pacote, mas pula a verificação de hashes da fonte
- `makepkg -c` : limpa os diretórios de trabalho após uma compilação bem sucedida
- `makepkg --verifysource` : verifica os hashes das fontes
- `makepkg --printsrcinfo >.SRCINFO` : gera e salva as informações da fonte no arquivo `.SRCINFO`

## Links
- More information: https://manned.org/makepkg
