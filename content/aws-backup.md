# aws backup

Unified backup service designed to protect Amazon Web Services and their associated data.

## Usage
- `aws backup get-backup-plan --backup-plan-id backup_plan_id` : return BackupPlan details for a specific BackupPlanId
- `aws backup create-backup-plan --backup-plan plan` : create a backup plan using a specific backup plan name and backup rules
- `aws backup delete-backup-plan --backup-plan-id backup_plan_id` : delete a specific backup plan
- `aws backup list-backup-plans` : list all active backup plans for the current account
- `aws backup list-report-jobs` : display details about your report jobs

## Categories
cloud, system

## Links
- More information: https://docs.aws.amazon.com/cli/latest/reference/backup/
- tldr page: https://tldr.inbrowser.app/pages/common/aws-backup
