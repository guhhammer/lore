# aws

Oficjalne narzędzie CLI dla Amazon Web Services. Niektóre podkomendy takie jak `s3` mają osobną dokumentację.

## Usage
- `aws configure wizard` : konfiguruj AWS Command-line
- `aws configure sso` : konfiguruj AWS Command-line używając SSO
- `aws sts get-caller-identity` : uzyskaj tożsamość wywołującego (służy do rozwiązywania problemów z uprawnieniami)
- `aws dynamodb list-tables --region us-east-1 --output yaml` : uzyskaj listę zasobów AWS w regionie i wyświetl ją w YAML
- `aws iam create-user --cli-auto-prompt` : użyj auto prompt do pomocy z poleceniem
- `aws dynamodb wizard nowa_tabela` : uzyskaj interaktywnego kreatora dla zasobu AWS
- `aws dynamodb update-table --generate-cli-skeleton` : generuj JSON CLI Skeleton (przydatne dla infrastruktury jako kodu)
- `aws komenda help` : wyświetl pomoc dla określonej komendy

## Links
- More information: https://docs.aws.amazon.com/cli/latest/reference/
