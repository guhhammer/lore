# echo

Imprime os argumentos passados.

## Usage
- `echo "Olá Mundo"` : imprime uma mensagem de texto. Nota: aspas são opcionais
- `echo "Meu caminho é $PATH"` : imprime uma mensagem com variáveis de ambiente
- `echo -n "Olá Mundo"` : imprime uma mensagem sem adicionar uma nova linha no final
- `echo "Olá Mundo" >>arquivo.txt` : adiciona uma mensagem no arquivo
- `echo -e "Coluna 1\tColuna 2"` : habilita interpretação dos códigos de escape após barra invertida (caracteres especiais)
- `echo $?` : imprime o status de saída do último comando executado (Nota: no prompt de comando do Windows e no PowerShell, os comandos equivalentes são `echo %errorlevel%` e `$lastexitcode` respectivamente)

## Links
- More information: https://www.gnu.org/software/coreutils/manual/html_node/echo-invocation.html
