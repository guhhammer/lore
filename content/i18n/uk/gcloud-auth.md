# gcloud auth

Видача та скасування авторизації для `gcloud` і керування обліковими даними.

## Usage
- `gcloud auth login` : авторизувати доступ до Google Cloud для `gcloud` CLI за допомогою облікових даних користувача Google Cloud і встановити поточний обліковий запис як активний
- `gcloud auth activate-service-account` : авторизувати доступ до Google Cloud, подібно до `gcloud auth login`, але за допомогою облікових даних сервісного облікового запису
- `gcloud auth application-default` : керувати Application Default Credentials (ADC) для хмарних клієнтських бібліотек
- `gcloud auth list` : вивести список облікових записів Google Cloud, які зараз автентифіковані у вашій системі
- `gcloud auth print-access-token` : вивести токен (token) доступу поточного облікового запису
- `gcloud auth revoke` : видалити облікові дані доступу до облікового запису

## Links
- More information: https://docs.cloud.google.com/sdk/gcloud/reference/auth
