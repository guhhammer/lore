# ssh-keygen

Gera chaves SSH usadas para autenticação, logins sem senha e outras finalidades.

## Usage
- `ssh-keygen` : gera uma chave interativamente
- `ssh-keygen -t ed25519 -a 32 -f ~/.ssh/nome_do_arquivo` : gera uma chave ed25519 com 32 rounds de função de derivação de chave e salva a chave em um arquivo específico
- `ssh-keygen -t rsa -b 4096 -C "comentário|email"` : gera uma chave RSA de 4096 bits com um comentário de email
- `ssh-keygen -R servidor_remoto` : remove as chaves de um servidor do arquivo known_hosts (útil quando um servidor conhecido tem uma nova chave)
- `ssh-keygen -l -E md5 -f ~/.ssh/nome_do_arquivo` : obtém a impressão digital de uma chave em MD5 Hex
- `ssh-keygen -p -f ~/.ssh/nome_do_arquivo` : altera a senha de uma chave
- `ssh-keygen -p -N "" -m PEM -f ~/.ssh/chave_privada_OpenSSH` : altera o tipo de formato da chave (por exemplo, de formato OPENSSH para PEM), o arquivo será reescrito no local
- `ssh-keygen -y -f ~/.ssh/chave_privada_OpenSSH` : obtém a chave pública a partir da chave secreta

## Links
- More information: https://man.openbsd.org/ssh-keygen
