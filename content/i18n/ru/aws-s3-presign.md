# aws s3 presign

Генерировать предподписанные URL-адреса для объектов Amazon S3.

## Usage
- `aws s3 presign s3://имя_бакета/путь/к/файлу` : сгенерировать предподписанный URL-адрес для конкретного объекта S3, действительный в течение одного часа
- `aws s3 presign s3://имя_бакета/путь/к/файлу --expires-in длительность_в_секундах` : сгенерировать предподписанный URL-адрес, действительный в течение указанного времени
- `aws s3 presign help` : показать справку

## Links
- More information: https://docs.aws.amazon.com/cli/latest/reference/s3/presign.html
