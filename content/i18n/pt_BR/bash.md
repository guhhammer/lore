# bash

Bourne-Again SHell, um interpretador de linha de comando compatível com `sh`.

## Usage
- `bash` : inicia uma sessão interativa do shell
- `bash --norc` : inicia uma sessão interativa do shell sem carregar as configurações de inicialização
- `bash -c "echo 'bash é executado'"` : executa comandos específicos
- `bash caminho/para/script.sh` : executa um script específico
- `bash -x caminho/para/script.sh` : executa um script específico exibindo cada comando antes de executá-lo
- `bash -e caminho/para/script.sh` : executa um script específico e para no primeiro erro
- `echo "echo 'bash é executado'" | bash` : executa comandos específicos da `stdin`
- `bash -r` : inicia uma sessão do shell restrita

## Links
- More information: https://www.gnu.org/software/bash/manual/bash.html#Invoking-Bash
