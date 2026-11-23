# curl

Transfere dados entre o computador local e um servidor. Suporta a maioria dos protocolos de comunicação, incluindo HTTP, HTTPS, FTP, SCP, etc.

## Usage
- `curl https://example.com` : faz um pedido HTTP GET e descarrega os conteúdos em `stdout` (saída padrão)
- `curl -L -D - https://example.com` : faz um pedido HTTP GET, segue redirecionamentos `3xx` e descarrega os cabeçalhos da resposta e conteúdos para `stdout`
- `curl -O https://example.com/arquivo.zip` : baixa um arquivo, salvando a saída no arquivo indicado pela URL
- `curl -X POST -d 'nome=maria' http://example.com/formulario` : envia dados codificados por formulário (pedido POST do tipo `application/x-www-form-urlencoded`). Usa `--data @file_name` ou `--data @'-'` para ler da `stdin`
- `curl -k -x http://127.0.0.1:8080 -H 'Authorization: Bearer token' -X GET|PUT|POST|DELETE|PATCH|... https://example.com` : envia um pedido com um cabeçalho adicional, usando um método HTTP personalizado e por meio de um proxy (tal como BurpSuite), ignorando certificados autoassinados inseguros
- `curl -d '{"nome":"maria"}' -H 'Content-Type: application/json' http://example.com/usuarios/1234` : envia dados no formato JSON, especificando o cabeçalho de tipo de conteúdo (content-type) apropriado
- `curl -E cliente.pem --key chave.pem -k https://example.com` : passa o certificado do cliente e chave para um recurso, pulando a validação do certificado
- `curl -v --resolve example.com:80:127.0.0.1 http://example.com` : resolve um hostname para um endereço de IP personalizado, com a saída verbosa (similar a editar o arquivo `/etc/hosts` para resolução de DNS personalizada)

## Links
- More information: https://curl.se/docs/manpage.html
