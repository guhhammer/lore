# dash

Debian Almquist Shell, uma implementação moderna e compatível com POSIX de `sh` (não compatível com Bash).

## Usage
- `dash` : inicia uma sessão de shell interativa
- `dash -c "echo 'dash executado'"` : executa comandos específicos
- `dash caminho/para/script.sh` : executa um script específico
- `dash -n caminho/para/script.sh` : checar erros de sintaxe em um script específico
- `dash -x caminho/para/script.sh` : executa comandos de um script, imprimindo cada comando antes de executá-lo
- `dash -e caminho/para/script.sh` : executa comandos de um script, parando no primeiro erro
- `echo "echo 'dash executado'" | dash` : executa comandos específicos de `stdin`

## Links
- More information: https://manned.org/dash
