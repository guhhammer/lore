# sudo

Executa um único comando como o Superuser, ou como outro usuário.

## Usage
- `sudo less /var/log/syslog` : executa um comando como Superuser
- `sudo -e /etc/fstab` : edita um arquivo, como Superuser, com seu editor padrão
- `sudo -u usuário -g grupo id -a` : executa um comando como outro usuário e/ou grupo
- `sudo !!` : executa um comando anterior com o prefixo `sudo` (apenas em Bash, Zsh, etc.)
- `sudo -i` : abre o shell padrão com privilégios de Superuser e executa arquivos de login (`.profile`, `.bash_profile`, etc.)
- `sudo -s` : abre o shell padrão com privilégios de Superuser sem altera o ambiente de execução
- `sudo -i -u usuário` : abre o shell padrão como dado usuário, carregando o ambiente de execução deste usuário e lendo arquivos de login (`.profile`, `.bash_profile`, etc.)
- `sudo -ll` : lista os comandos permitidos (e não permitidos) para o usuário atual

## Links
- More information: https://www.sudo.ws/sudo.html
