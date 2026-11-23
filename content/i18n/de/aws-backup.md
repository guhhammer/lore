# aws backup

Einheitliches Backup-Service zum Schutz der Amazon Web Services und der damit verbundenen Daten.

## Usage
- `aws backup get-backup-plan --backup-plan-id id` : gib Backup-Plan-Details für eine bestimmte Backup-Plan-ID aus
- `aws backup create-backup-plan --backup-plan plan` : erstelle einen Backup-Plan unter Verwendung eines bestimmten Backup-Plan-Namens und von Backup-Regeln
- `aws backup delete-backup-plan --backup-plan-id id` : lösche einen bestimmten Backup-Plan
- `aws backup list-backup-plans` : gib eine Liste aller aktiven Backup-Pläne für das aktuelle Konto aus
- `aws backup list-report-jobs` : zeige Details über die Report-Aufträge an

## Links
- More information: https://docs.aws.amazon.com/cli/latest/reference/backup/
