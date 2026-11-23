# wget

Baixar arquivos da Internet. Suporta HTTP, HTTPS, e FTP.

## Usage
- `wget https://example.com/foo` : baixa o conteúdo de uma URL para o arquivo (nomeado como "foo" neste caso)
- `wget -O caminho/para/arquivo https://example.com/foo` : baixa o conteúdo de uma URL para o arquivo (nomeado como "bar" neste caso)
- `wget -pkw 3 https://example.com/algumapagina.html` : baixa uma única página web e todo os seus recursos com intervalos de 3 segundos entre requisições (scripts, stylesheets, imagens, etc.)
- `wget -mnp https://example.com/algumcaminho/` : baixa todos os arquivos listados dentro de um diretório e seus sub-diretórios (não baixa elementos de página incorporados)
- `wget --limit-rate 300k -t 100 https://example.com/algumcaminho/` : limita a velocidade de download e o número de novas tentativas de conexão
- `wget --user nomeusuario --password senha https://example.com` : baixa um arquivo de um servidor HTTP usando Autenticação Básica (também funciona para FTP)
- `wget -c https://example.com` : continua um download incompleto
- `wget -P caminho/para/diretorio -i URLs.txt` : baixa todas as URLs armazenadas em um arquivo de texto para um diretório específico

## Links
- More information: https://www.gnu.org/software/wget/manual/wget.html
