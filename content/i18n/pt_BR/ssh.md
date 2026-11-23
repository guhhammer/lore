# ssh

O Secure Shell é um protocolo usado para fazer login de forma segura em sistemas remotos. Ele pode ser usado para fazer login ou executar comandos em um servidor remoto.

## Usage
- `ssh nome_do_usuário@servidor_remoto` : conecta a um servidor remoto
- `ssh -i caminho/para/arquivo_de_chave nome_do_usuário@servidor_remoto` : conecta a um servidor remoto com uma identidade específica (chave privada)
- `ssh nome_do_usuário@servidor_remoto -p 2222` : conecta a um servidor remoto usando uma porta específica
- `ssh nome_do_usuário@servidor_remoto -t comando argumentos_do_comando` : executa um comando em um servidor remoto com uma alocação de tty permitindo interação com o comando remoto
- `ssh -D 1080 nome_do_usuário@servidor_remoto` : tunelamento SSH: Encaminhamento dinâmico de porta (proxy SOCKS em `localhost:1080`)
- `ssh -L 9999:example.org:80 -N -T nome_do_usuário@servidor_remoto` : tunelamento SSH: Encaminha uma porta específica (`localhost:9999` para `example.org:80`), desativa a alocação de pseudo-tty e execução de comandos remotos
- `ssh -J nome_do_usuário@servidor_intermediário nome_do_usuário@servidor_remoto` : salta com SSH: Conecta a um servidor remoto através de um servidor intermediário (vários saltos intermediários podem ser especificados separados por vírgula)
- `<Enter><~><.>` : fecha uma sessão travada

## Links
- More information: https://man.openbsd.org/ssh
