# aws s3 rb

Удалять пустой бакет S3.

## Usage
- `aws s3 rb s3://имя_бакета` : удалить пустой бакет S3
- `aws s3 rb s3://имя_бакета --force` : принудительно удалить бакет S3 и его неверсионированные объекты (не удастся, если в бакете есть версионированные объекты)

## Links
- More information: https://docs.aws.amazon.com/cli/latest/reference/s3/rb.html
