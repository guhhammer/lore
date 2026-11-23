# aws-vault

Un vault per memorizzare e accedere in modo sicuro alle credenziali AWS in ambienti di sviluppo.

## Usage
- `aws-vault add profilo` : aggiunge credenziali al keystore sicuro
- `aws-vault exec profilo -- aws s3 ls` : esegue un comando con credenziali AWS nell'ambiente
- `aws-vault login profilo` : apre una finestra del browser e accede alla Console AWS
- `aws-vault list` : elenca i profili, insieme alle loro credenziali e sessioni
- `aws-vault rotate profilo` : ruota le credenziali AWS
- `aws-vault remove profilo` : rimuove le credenziali dal keystore sicuro

## Links
- More information: https://github.com/99designs/aws-vault
