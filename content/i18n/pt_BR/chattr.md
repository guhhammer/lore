# chattr

Altera os atributos de arquivos ou diretórios.

## Usage
- `chattr +i caminho_do_arquivo_ou_diretorio` : bloqueia um arquivo ou diretório para mudanças ou remoção, mesmo para um super usuário
- `chattr -i caminho_do_arquivo_ou_diretorio` : desbloqueia um arquivo ou diretório
- `chattr -R +i caminho_do_diretorio` : bloqueia diretório e todos os seus arquivos para mudanças ou remoção

## Links
- More information: https://manned.org/chattr
