# ibmcloud login

Входить в IBM Cloud.

## Usage
- `ibmcloud login` : войти в интерактивном режиме
- `ibmcloud login -a конечная_точка_api` : войти, указав конкретную конечную точку API (по умолчанию `cloud.ibm.com`)
- `ibmcloud login -u имя_пользователя -p пароль -r us-south` : войти, указав имя пользователя, пароль и целевой регион в качестве параметров
- `ibmcloud login --apikey строка_api_ключа` : войти с использованием API-ключа, передав его как аргумент
- `ibmcloud login --apikey @путь/к/файлу_api_ключа` : войти с использованием API-ключа, передав его как файл
- `ibmcloud login --sso` : войти с использованием федеративного идентификатора (единый вход)

## Links
- More information: https://cloud.ibm.com/docs/cli?topic=cli-ibmcloud_cli#ibmcloud_login
