# dexter

Strumento per autenticare utenti kubectl con OpenId Connect.

## Usage
- `dexter auth -i id_client -s segreto_client` : crea ed autentica un utente con Google OIDC
- `dexter auth -i id_client -s segreto_client -k percorso/della/configurazione` : sovrascrivi la posizione predefinita della configurazione di kube

## Links
- More information: https://github.com/gini/dexter#run-dexter
