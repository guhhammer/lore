# aws s3 website

Настраивать конфигурацию веб-сайта для бакета.

## Usage
- `aws s3 website s3://имя-бакета --index-document index.html` : настроить бакет как статический веб-сайт
- `aws s3 website s3://имя-бакета --index-document index.html --error-document error.html` : настроить страницу ошибки для веб-сайта

## Links
- More information: https://docs.aws.amazon.com/cli/latest/reference/s3/website.html
