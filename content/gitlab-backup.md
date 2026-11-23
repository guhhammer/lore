# gitlab-backup

Manage backups for a GitLab Omnibus installation.

## Usage
- `sudo gitlab-backup create` : create a full backup (default strategy)
- `sudo gitlab-backup create STRATEGY=copy` : create a full backup using the copy strategy
- `sudo gitlab-backup restore BACKUP=backup_id` : restore a backup by specifying its ID
- `sudo gitlab-backup restore BACKUP=backup_id SKIP=db,uploads,...` : restore a backup while skipping specific components

## See also
- gitlab-ctl
- gitlab-runner

## Categories
users

## Links
- More information: https://docs.gitlab.com/administration/backup_restore/backup_gitlab/
- tldr page: https://tldr.inbrowser.app/pages/common/gitlab-backup
