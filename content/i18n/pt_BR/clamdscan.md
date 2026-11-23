# clamdscan

Faz uma varredura em busca de vírus usando o ClamAV Daemon.

## Usage
- `clamdscan caminho/para/arquivo_ou_diretório` : faz uma varredura em um arquivo ou diretório por vulnerabilidades
- `comando | clamdscan -` : faz uma varredura nos dados da `stdin` (entrada padrão)
- `clamdscan --infected` : faz uma varredura no diretório atual e lista apenas os arquivos infectados
- `clamdscan --log caminho/para/arquivo_de_registro` : gera um relatório da varredura para um arquivo de registro
- `clamdscan --move caminho/para/diretório_de_quarentena` : move arquivos infectados para um diretório específico
- `clamdscan --remove` : remove arquivos infectados
- `clamdscan --multiscan` : usa várias threads para para fazer uma varredura em um diretório
- `clamdscan --fdpass` : passa o descritor de arquivo em vez de transmitir o arquivo para o daemon

## Links
- More information: https://docs.clamav.net/manual/Usage/Scanning.html#clamdscan
