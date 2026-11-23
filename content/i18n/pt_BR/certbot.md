# certbot

O agente da Let's Encrypt para obtenção e renovação de certificados TLS automaticamente. Sucessor do `letsencrypt`.

## Usage
- `sudo certbot certonly --webroot -w caminho_para_webroot -d subdominio.dominio.com` : obtém um novo certificado via autorização webroot, porém sem instala-o automaticamente
- `sudo certbot --nginx -d subdominio.dominio.com` : obtém um novo certificado via autorização `nginx` e instala-o automaticamente
- `sudo certbot --apache -d subdominio.dominio.com` : obtém um novo certificado via autorização apache e instala-o automaticamente
- `sudo certbot renew` : renova todos os certificados que expirarão em 30 dias ou menos (não esqueça de reiniciar todos os servidores que usam os certificados)
- `sudo certbot --webroot -w caminho_para_webroot -d subdominio.dominio.com --dry-run` : simula a obtenção de um novo certificado, porém sem salvá-lo no disco rígido
- `sudo certbot --webroot -w caminho_para_webroot -d subdominio.dominio.com --test-cert` : obtém um certificado não confiável para testes

## Links
- More information: https://eff-certbot.readthedocs.io/en/latest/using.html
