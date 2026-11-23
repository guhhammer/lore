# gitlab-backup

Управлять резервными копиями установки GitLab Omnibus.

## Usage
- `sudo gitlab-backup create` : создать полную резервную копию (стратегия по умолчанию)
- `sudo gitlab-backup create STRATEGY=copy` : создать полную резервную копию, используя стратегию копирования
- `sudo gitlab-backup restore BACKUP=идентификатор_копии` : восстановить резервную копию, указав её идентификатор
- `sudo gitlab-backup restore BACKUP=идентификатор_копии SKIP=db,uploads,...` : восстановить резервную копию, пропустив конкретные компоненты

## Links
- More information: https://docs.gitlab.com/administration/backup_restore/backup_gitlab/
