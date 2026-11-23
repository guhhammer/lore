# aws backup

Servizio unificato di backup progettato per proteggere i servizi Amazon Web Services e i relativi dati.

## Usage
- `aws backup get-backup-plan --backup-plan-id id` : restituisce i dettagli di BackupPlan per un BackupPlanId specifico
- `aws backup create-backup-plan --backup-plan plan` : crea un piano di backup utilizzando un nome di piano di backup specifico e regole di backup
- `aws backup delete-backup-plan --backup-plan-id id` : elimina un piano di backup specifico
- `aws backup list-backup-plans` : elenca tutti i piani di backup attivi per l'account corrente
- `aws backup list-report-jobs` : visualizza i dettagli sui lavori di report

## Links
- More information: https://docs.aws.amazon.com/cli/latest/reference/backup/
