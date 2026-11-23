# ippeveprinter

Um servidor de impressão IPP Everywhere simples.

## Usage
- `ippeveprinter "nome_do_serviço"` : executa o servidor com um nome de serviço específico
- `ippeveprinter -P caminho/para/arquivo.ppd "nome_do_serviço"` : carrega os atributos da impressora de um arquivo PPD
- `ippeveprinter -c /usr/bin/file "nome_do_serviço"` : executa o comando `file` sempre que um trabalho é enviado para o servidor
- `ippeveprinter -d diretório_spool "nome_do_serviço"` : especifica o diretório que vai conter os arquivos de impressão (por padrão, um diretório dentro do diretório temporário do usuário)
- `ippeveprinter -k "nome_do_serviço"` : mantém os documentos de impressão no diretório de spool em vez de exclui-los
- `ippeveprinter -s velocidade "nome_do_serviço"` : especifica a velocidade da impressora na unidade páginas/minuto (10 por padrão)

## Links
- More information: https://openprinting.github.io/cups/doc/man-ippeveprinter.html
