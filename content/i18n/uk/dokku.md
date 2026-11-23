# dokku

Міні-Heroku на основі Docker (PaaS). Легко розгортає кілька програм на власному сервері різними мовами за допомогою однієї команди `git-push`.

## Usage
- `dokku apps` : показати запущені програми
- `dokku apps:create ім'я_програми` : створити програму
- `dokku apps:destroy ім'я_програми` : видалити програму
- `dokku plugin:install повний_url_на_репозиторій` : встановити плагін
- `dokku db:link ім'я_бази_даних ім'я_програми` : зв'язати базу даних із програмою

## Links
- More information: https://dokku.com/docs/deployment/application-deployment/
