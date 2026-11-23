# gitlab-ctl

Управлять GitLab Omnibus.

## Usage
- `sudo gitlab-ctl status` : показать статус всех сервисов
- `sudo gitlab-ctl status nginx` : показать статус конкретного сервиса
- `sudo gitlab-ctl restart` : перезапустить все сервисы
- `sudo gitlab-ctl restart nginx` : перезапустить конкретный сервис
- `sudo gitlab-ctl tail` : показать логи всех сервисов и продолжать чтение до нажатия `<Ctrl c>`
- `sudo gitlab-ctl tail nginx` : показать логи конкретного сервиса
- `sudo gitlab-ctl kill nginx` : отправить сигнал `SIGKILL` конкретному сервису
- `sudo gitlab-ctl reconfigure` : перенастроить приложение

## Links
- More information: https://docs.gitlab.com/omnibus/maintenance/
