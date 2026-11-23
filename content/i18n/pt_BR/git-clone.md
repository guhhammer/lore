# git clone

Clona um repositório existente.

## Usage
- `git clone local_do_repositório_remoto caminho/para/diretório` : clona um repositório existente em um novo diretório (o diretório padrão é o nome do repositório)
- `git clone --recursive local_do_repositório_remoto` : clona um repositório existente e seus submódulos
- `git clone -n local_do_repositório_remoto` : clona somente o diretório `.git` de um repositório existente
- `git clone -l caminho/para/repositório/local` : clona um repositório local
- `git clone -q local_do_repositório_remoto` : clona de forma silenciosa
- `git clone --depth 10 local_do_repositório_remoto` : clona um repositório existente buscando somente os 10 commits mais recentes na branch padrão (útil para salvar tempo)
- `git clone -b nome --single-branch local_do_repositório_remoto` : clona um repositório existente buscando somente uma branch específica
- `git clone -c core.sshCommand="ssh -i caminho/para/chave_ssh_privada" local_do_repositório_remoto` : clona um repositório existente usando um comando SSH específico

## Links
- More information: https://git-scm.com/docs/git-clone
