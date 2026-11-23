# velero

Back up and migrate Kubernetes applications and their persistent volumes.

## Usage
- `velero backup create backup_name` : create a backup containing all resources
- `velero backup get` : list all backups
- `velero backup delete backup_name` : delete a backup
- `velero schedule create schedule_name --schedules="@every 7d" --ttl 2160h0m0s` : create a weekly backup, each living for 90 days (2160 hours)
- `velero restore create --from-schedule schedule_name` : create a restore from the latest successful backup triggered by specific schedule

## Categories
containers, system

## Links
- More information: https://velero.io/docs/main/
- tldr page: https://tldr.inbrowser.app/pages/common/velero
