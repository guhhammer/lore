# acme.sh-dns

Utiliza un desafío DNS-01 para emitir un certificado TLS.

## Usage
- `acme.sh --issue --dns dns_gd --domain example.com` : emite un certificado utilizando un modo API DNS automático
- `acme.sh --issue --dns dns_namesilo --domain example.com --domain *.example.com` : emite un certificado comodín (marcado con un asterisco) utilizando un modo API DNS automático
- `acme.sh --issue --dns dns_cf --domain example.com --challenge-alias alias-para-ejemplo-validacion.com` : emite un certificado utilizando un modo de alias DNS
- `acme.sh --issue --dns dns_namecheap --domain example.com --dnssleep 300` : emite un certificado mientras se desactiva el sondeo automático de Cloudflare / Google DNS después de añadir el registro DNS especificando un tiempo de espera personalizado en segundos
- `acme.sh --issue --dns --domain example.com --yes-I-know-dns-manual-mode-enough-go-ahead-please` : emite un certificado utilizando un modo DNS manual

## Links
- More information: https://github.com/acmesh-official/acme.sh/wiki
