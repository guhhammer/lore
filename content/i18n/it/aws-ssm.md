# aws ssm

Interagisce in modo sicuro con le risorse AWS e le gestisce. Nota: le sessioni interattive richiedono che il Session Manager plugin sia installato.

## Usage
- `aws ssm send-command --instance-ids id_istanza1 id_istanza2 ... --document-name "AWS-RunShellScript" --parameters 'commands=["comando"]'` : esegue comandi sulle istanze
- `aws ssm list-command-invocations --instance-id "id_istanza"` : controlla le esecuzioni dei comandi su un'istanza
- `aws ssm list-command-invocations --command-id "id_comando" --details` : controlla l'output di un comando specifico
- `aws ssm start-session --target "id_istanza"` : avvia una sessione interattiva con un'istanza
- `aws ssm start-session --target "id_istanza" --document-name "AWS-StartPortForwardingSessionToRemoteHost" --parameters '{"portNumber":["porta_remota"],"localPortNumber":["porta_locale"]}'` : avvia una sessione di port forwarding verso un host remoto

## Links
- More information: https://docs.aws.amazon.com/cli/latest/reference/ssm/
