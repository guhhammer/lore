# chown

Muda o usuário e grupo donos de arquivos e diretórios.

## Usage
- `chown usuario caminho/para/arquivo_ou_diretório` : muda o usuário que é dono de um arquivo ou diretório
- `chown usuario:grupo caminho/para/arquivo_ou_diretório` : muda o usuário e grupo que são donos de um arquivo/diretório
- `chown usuario: caminho/para/arquivo_ou_diretorio` : muda o usuário dono e o grupo para que ambos tenha o nome `usuario`
- `chown -R usuario caminho/para/diretório` : recursivamente muda o dono de um diretório e seu conteúdo
- `chown -h usuario caminho/para/symlink` : muda o dono de um link simbólico
- `chown --reference caminho/para/arquivo_de_referência caminho/para/arquivo_ou_diretório` : muda o dono de um arquivo/diretório para ficar igual a um arquivo de referência
