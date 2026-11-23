# aws rds

Utilizza AWS Relational Database Service, un servizio web per impostare, operare e scalare database relazionali.

## Usage
- `aws rds subcommand help` : visualizza l'aiuto per un sottocomando RDS specifico
- `aws rds stop-db-instance --db-instance-identifier identificativo_istanza` : ferma l'istanza
- `aws rds start-db-instance --db-instance-identifier identificativo_istanza` : avvia l'istanza
- `aws rds modify-db-instance --db-instance-identifier identificativo_istanza parametri --apply-immediately` : modifica un'istanza RDS
- `aws rds apply-pending-maintenance-action --resource-identifier database_arn --apply-action system-update --opt-in-type immediate` : applica aggiornamenti a un'istanza RDS
- `aws rds modify-db-instance --db-instance-identifier old_identificativo_istanza --new-db-instance-identifier nuovo_identificativo_istanza` : cambia l'identificativo dell'istanza
- `aws rds reboot-db-instance --db-instance-identifier identificativo_istanza` : riavvia l'istanza
- `aws rds delete-db-instance --db-instance-identifier identificativo_istanza --final-db-snapshot-identifier identificativo_snapshot --delete-automated-backups` : elimina l'istanza

## Links
- More information: https://docs.aws.amazon.com/cli/latest/reference/rds/
