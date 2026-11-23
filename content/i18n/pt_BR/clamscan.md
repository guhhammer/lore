# clamscan

Um antivírus de linha de comando.

## Usage
- `clamscan caminho/para/arquivo` : faz uma varredura em um arquivo por vulnerabilidades
- `clamscan -r caminho/para/diretório` : faz uma varredura em todos os arquivos recursivamente em um diretório específico
- `command | clamscan -` : faz uma varredura nos dados da `stdin` (entrada padrão)
- `clamscan -d caminho/para/diretório_ou_arquivo_banco_de_dados` : especifica um arquivo de banco de dados de vírus ou diretório de arquivos
- `clamscan -i` : faz uma varredura no diretório atual e lista apenas os arquivos infectados
- `clamscan -l caminho/para/arquivo_de_registro` : gera um relatório da varredura para um arquivo de registro
- `clamscan --move caminho/para/diretório_de_quarentena` : move arquivos infectados para um diretório específico
- `clamscan --remove yes` : remove arquivos infectados

## Links
- More information: https://docs.clamav.net/manual/Usage/Scanning.html#clamscan
