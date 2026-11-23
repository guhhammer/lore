# run0

Eleva privilégios interativamente. Similar ao `sudo`, mas não é um binário SUID, a autenticação acontece via polkit, e comandos são executados a partir de um serviço do `systemd`.

## Usage
- `run0 comando` : executa um comando como root
- `run0 -u usuário|uid -g nome_do_grupo|gid comando` : executa um comando como outro usuário e/ou grupo

## Links
- More information: https://www.freedesktop.org/software/systemd/man/latest/run0.html
