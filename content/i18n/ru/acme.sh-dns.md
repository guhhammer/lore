# acme.sh-dns

Использовать проверку DNS-01 для выпуска TLS-сертификата.

## Usage
- `acme.sh --issue --dns dns_gd --domain example.com` : выпустить сертификат, используя автоматический режим DNS API
- `acme.sh --issue --dns dns_namesilo --domain example.com --domain *.example.com` : выпустить wildcard-сертификат (отмеченный звёздочкой), используя автоматический режим DNS API
- `acme.sh --issue --dns dns_cf --domain example.com --challenge-alias alias-for-example-validation.com` : выпустить сертификат, используя режим DNS-алиаса (псевдонима)
- `acme.sh --issue --dns dns_namecheap --domain example.com --dnssleep 300` : выпустить сертификат, отключив автоматический опрос DNS Cloudflare/Google после добавления DNS-записи и указав время ожидания в секундах
- `acme.sh --issue --dns --domain example.com --yes-I-know-dns-manual-mode-enough-go-ahead-please` : выпустить сертификат, используя ручной режим DNS

## Links
- More information: https://github.com/acmesh-official/acme.sh/wiki
