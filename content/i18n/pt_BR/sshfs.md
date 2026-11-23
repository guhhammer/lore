# sshfs

Cliente de sistema de arquivos baseado em SSH.

## Usage
- `sshfs nome_do_usuário@servidor_remoto:diretório_remoto ponto_de_montagem` : monta um diretório remoto
- `umount ponto_de_montagem` : desmonta um diretório remoto
- `sshfs nome_do_usuário@servidor_remoto:diretório_remoto -p 2222` : monta um diretório remoto de um servidor com uma porta específica
- `sshfs nome_do_usuário@servidor_remoto:diretório_remoto -C` : usa compressão
- `sshfs -o follow_symlinks nome_do_usuário@servidor_remoto:diretório_remoto ponto_de_montagem` : segue links simbólicos

## Links
- More information: https://github.com/libfuse/sshfs/blob/master/sshfs.rst
