# aws ses

CLI per AWS Simple Email Service. Servizio email cloud in ingresso e in uscita ad alta scala.

## Usage
- `aws ses create-receipt-rule-set --rule-set-name nome_set_regole --generate-cli-skeleton` : crea un nuovo set di regole di ricezione
- `aws ses describe-active-receipt-rule-set --generate-cli-skeleton` : descrivi il set di regole di ricezione attivo
- `aws ses describe-receipt-rule --rule-set-name nome_set_regole --rule-name nome_regola --generate-cli-skeleton` : descrivi una regola di ricezione specifica
- `aws ses list-receipt-rule-sets --starting-token token_string --max-items integer --generate-cli-skeleton` : elenca tutti i set di regole di ricezione
- `aws ses delete-receipt-rule-set --rule-set-name nome_set_regole --generate-cli-skeleton` : elimina un set di regole di ricezione specifico (il set di regole attualmente attivo non può essere eliminato)
- `aws ses delete-receipt-rule --rule-set-name nome_set_regole --rule-name nome_regola --generate-cli-skeleton` : elimina una regola di ricezione specifica
- `aws ses send-email --from from_address --destination "ToAddresses=addresses" --message "Subject={Data=subject_text,Charset=utf8},Body={Text={Data=body_text,Charset=utf8},Html={Data=message_body_containing_html,Charset=utf8\}\}"` : invia un'email
- `aws ses subcommand help` : visualizza l'aiuto per un sottocomando SES specifico

## Links
- More information: https://docs.aws.amazon.com/cli/latest/reference/ses/
