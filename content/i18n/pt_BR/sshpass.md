# sshpass

Um provedor de senha SSH. Ele funciona criando um TTY, inserindo a senha nele e, em seguida, redirecionando `stdin` para a sessão SSH.

## Usage
- `sshpass -d 0 ssh usuário@nome_do_servidor` : conecta a um servidor remoto usando uma senha fornecida em um descritor de arquivo (neste caso, `stdin`)
- `sshpass -p senha ssh -o StrictHostKeyChecking=no usuário@nome_do_servidor` : conecta a um servidor remoto com a senha fornecida como opção e aceita automaticamente chaves SSH desconhecidas
- `sshpass -f caminho/para/arquivo ssh -o StrictHostKeyChecking=no usuário@nome_do_servidor "comando"` : conecta a um servidor remoto usando a primeira linha de um arquivo como senha, aceita automaticamente chaves SSH desconhecidas e executa um comando

## Links
- More information: https://manned.org/sshpass
