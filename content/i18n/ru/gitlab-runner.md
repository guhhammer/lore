# gitlab-runner

Управлять раннерами GitLab.

## Usage
- `sudo gitlab-runner register --url https://gitlab.example.com --registration-token токен --name имя` : зарегистрировать раннер
- `sudo gitlab-runner register --url https://gitlab.example.com --registration-token токен --name имя --executor docker` : зарегистрировать раннер с исполнителем Docker
- `sudo gitlab-runner unregister --name имя` : отменить регистрацию раннера
- `sudo gitlab-runner status` : показать статус сервиса раннера
- `sudo gitlab-runner restart` : перезапустить сервис раннера
- `sudo gitlab-runner verify` : проверить, могут ли зарегистрированные раннеры подключиться к GitLab

## Links
- More information: https://docs.gitlab.com/runner/
