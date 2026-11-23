# aws secretsmanager

Memorizza, gestisce e recupera segreti.

## Usage
- `aws secretsmanager list-secrets` : mostra i segreti memorizzati dal secrets manager nell'account corrente
- `aws secretsmanager list-secrets --query 'SecretList[*].{Name: Name, ARN: ARN}'` : elenca tutti i segreti ma mostra solo i nomi dei segreti e gli ARN (facile da visualizzare)
- `aws secretsmanager create-secret --name name --description "secret_description" --secret-string 'segreto'` : crea un segreto
- `aws secretsmanager delete-secret --secret-id nome|arn` : elimina un segreto (aggiungi `--force-delete-without-recovery` per eliminare immediatamente senza periodo di recupero)
- `aws secretsmanager describe-secret --secret-id nome|arn` : visualizza i dettagli di un segreto escluso il testo del segreto
- `aws secretsmanager get-secret-value --secret-id nome|arn --version-stage versione_del_segreto` : recupera il valore di un segreto (per ottenere l'ultima versione del segreto ometti `--version-stage`)
- `aws secretsmanager rotate-secret --secret-id nome|arn --rotation-lambda-arn arn_della_funzione_lambda` : ruota il segreto immediatamente utilizzando una funzione Lambda
- `aws secretsmanager rotate-secret --secret-id nome|arn --rotation-lambda-arn arn_della_funzione_lambda --rotation-rules AutomaticallyAfterDays=30` : ruota il segreto automaticamente ogni 30 giorni utilizzando una funzione Lambda

## Links
- More information: https://docs.aws.amazon.com/cli/latest/reference/secretsmanager/
