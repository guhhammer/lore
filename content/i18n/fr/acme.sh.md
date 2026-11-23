# acme.sh

Script shell implémentant le protocole client ACME, une alternative à `certbot`.

## Usage
- `acme.sh --issue -d example.com -w /chemin/vers/webroot` : publie un certificat en utilisant le mode webroot
- `acme.sh --issue --standalone -d example.com -d www.example.com` : publie un certificat pour plusieurs domaines en utilisant le mode autonome avec le port 80
- `acme.sh --issue --alpn -d example.com` : publie un certificat en utilisant le mode autonome TLS avec le port 443
- `acme.sh --issue --nginx -d example.com` : publie un certificat en utilisant une configuration `nginx`
- `acme.sh --issue --apache -d example.com` : publie un certificat en utilisant une configuration Apache
- `acme.sh --issue --dns dns_cf -d *.example.com` : publie un certificat wildcard (\*) en utilisant le mode automatique DNS API
- `acme.sh -i -d example.com --key-file /chemin/vers/example.com.key --fullchain-file /chemin/vers/example.com.cer --reloadcmd "systemctl force-reload nginx"` : installe les fichiers de certificat dans un répertoire spécifique (Utile pour les renouvellements automatiques de certificat)

## Links
- More information: https://github.com/acmesh-official/acme.sh#2-just-issue-a-cert
