# mvn deploy

Добавлять артефакт в удалённый репозиторий.

## Usage
- `mvn deploy` : скопировать итоговый артефакт в удалённый репозиторий, настроенный в файле `settings.xml`
- `mvn deploy:deploy-file -D url=url_удалённого_репозитория -D repositoryId=id_сервера_из_settings_xml -D file=путь/к/файлу` : скопировать артефакт, не собранный с помощью Maven, в удалённый репозиторий

## Links
- More information: https://manned.org/mvn
