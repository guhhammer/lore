# ibmcloud api

Устанавливать или просматривать конечную точку API IBM Cloud.

## Usage
- `ibmcloud api` : показать текущую конечную точку API
- `ibmcloud api cloud.ibm.com` : установить конечную точку API на `cloud.ibm.com`
- `ibmcloud api private.cloud.ibm.com` : установить приватную конечную точку API
- `ibmcloud api private.cloud.ibm.com --vpc` : использовать VPC-соединение для приватной конечной точки
- `ibmcloud api https://cloud.ibm.com --skip-ssl-validation` : пропустить проверку SSL для HTTP-запросов
- `ibmcloud api --unset` : удалить настройку конечной точки API

## Links
- More information: https://cloud.ibm.com/docs/cli?topic=cli-ibmcloud_cli#ibmcloud_api
