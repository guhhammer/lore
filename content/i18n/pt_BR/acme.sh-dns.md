# acme.sh-dns

Usa um desafio DNS-01 para emitir um certificado TLS.

## Usage
- `acme.sh --issue --dns dns_gd --domain example.com` : emite um certificado usando um modo de DNS API automático
- `acme.sh --issue --dns dns_namesilo --domain example.com --domain *.example.com` : emite um certificado wildcard (denotado por um asterísco (*)) usando um modo DNS API automático
- `acme.sh --issue --dns dns_cf --domain example.com --challenge-alias apelido-para-exemplo-de-validacao.com` : emite um certificado usando um modo apelido de DNS
- `acme.sh --issue --dns dns_namecheap --domain example.com --dnssleep 300` : emite um certificado enquanto desabilita a pesquisa automática de DNS da Cloudflare/Google depois que o registro DNS for adicionado, especificando um tempo de espera personalizado em segundos
- `acme.sh --issue --dns --domain example.com --yes-I-know-dns-manual-mode-enough-go-ahead-please` : emite um certificado usando o modo DNS manual

## Links
- More information: https://github.com/acmesh-official/acme.sh/wiki
