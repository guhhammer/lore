# aws cloud9

Gestisce Cloud9 - una raccolta di strumenti per codificare, costruire, eseguire, testare, debuggare e rilasciare software nel cloud.

## Usage
- `aws cloud9 list-environments` : elenca tutti gli identificativi degli ambienti di sviluppo Cloud9
- `aws cloud9 create-environment-ec2 --name nome --instance-type tipo_istanza` : crea un ambiente di sviluppo Cloud9
- `aws cloud9 describe-environments --environment-ids ids_environment` : visualizza informazioni sugli ambienti di sviluppo Cloud9
- `aws cloud9 create-environment-membership --environment-id id_environment --user-arn arn_utente --permissions permessi` : aggiunge un membro ambiente a un ambiente di sviluppo Cloud9
- `aws cloud9 describe-environment-status --environment-id id_environment` : visualizza informazioni sullo stato di un ambiente di sviluppo Cloud9
- `aws cloud9 delete-environment --environment-id id_environment` : elimina un ambiente Cloud9
- `aws cloud9 delete-environment-membership --environment-id id_environment --user-arn arn_utente` : elimina un membro ambiente da un ambiente di sviluppo

## Links
- More information: https://docs.aws.amazon.com/cli/latest/reference/cloud9/
