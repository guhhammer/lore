# dig

Utilitário de pesquisa de DNS.

## Usage
- `dig +short example.com` : pesquisa o(s) IP(s) associados a um hostname (registros A)
- `dig +noall +answer example.com` : obtém uma resposta detalhada para um determinado domínio (registros A)
- `dig +short example.com A|MX|TXT|CNAME|NS` : consulta um tipo de registro DNS específico associado a um nome de domínio fornecido
- `dig +tls @1.1.1.1|8.8.8.8|9.9.9.9|... example.com` : especifica um DNS alternativo para busca e opcionalmente usa DNS sobre TLS (DoT)
- `dig -x 8.8.8.8` : performa uma busca reversa de DNS em um endereço de IP (registro PTR)
- `dig +nssearch example.com` : encontra servidores de nomes autorizados para a região e exibe os registros SOA
- `dig +trace example.com` : performa consultas iterativas e exibe o caminho de ratreio completo para resolver um nome de domínio
- `dig +tcp -p porta @ip_servidor_dns example.com` : busca um servidor DNS sobre uma porta não padrão usando protocolo TCP

## Links
- More information: https://manned.org/dig
