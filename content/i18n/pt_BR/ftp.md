# ftp

Ferramentas para interagir com um servidor via Protocolo de Transferência de Arquivos.

## Usage
- `ftp ftp.example.com` : conecta-se a um servidor FTP
- `ftp endereço_ip porta` : conecta a um servidor FTP especificando o endereço de IP e porta
- `binary` : [Interativo] Alterna para o modo de transferência binária (gráficos, arquivos compactados, etc)
- `prompt off` : [Interativo] Transfere vários arquivos sem solicitar confirmação em cada arquivo
- `mget *.png` : [Interativo] Baixa vários arquivos (expressão `glob`)
- `mput *.zip` : [Interativo] Carrega vários arquivos (expressão `glob`)
- `mdelete *.txt` : [Interativo] Exclui vários arquivos do servidor remoto
- `rename nome_do_arquivo_original novo_nome_do_arquivo` : [Interativo] Renomeia um arquivo no servidor remoto

## Links
- More information: https://manned.org/ftp
