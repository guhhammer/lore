# ab

Ferramenta da Apache para realizar benchmarking e testes de carga em servidores web.

## Usage
- `ab -n 100 url` : executa 100 requisições HTTP do tipo GET para uma determinada URL
- `ab -n 100 -c 10 url` : executa 100 requisições HTTP do tipo GET para uma determinada URL, executando 10 requisições simultâneas de cada vez
- `ab -n 100 -T application/json -p caminho/para/arquivo.json url` : executa 100 requisições HTTP do tipo POST para uma determinada URL, usando um payload JSON de um arquivo
- `ab -k url` : utiliza a funcionalidade HTTP Keep Alive, permitindo que várias requisições sejam feitas em uma sessão HTTP
- `ab -t 60 url` : define o tempo total do benchmarking, em segundos
- `ab -e caminho/para/arquivo.csv` : escreve os resultados em um arquivo CSV

## Links
- More information: https://httpd.apache.org/docs/current/programs/ab.html
