# aws cur

Crea, solicita y elimina definiciones de informes de uso de AWS.

## Usage
- `aws cur put-report-definition --report-definition file://ruta/al/report_definition.json` : crea una definición de informe de costes y uso de AWS a partir de un archivo JSON
- `aws cur describe-report-definitions` : enumera las definiciones de informes de uso definidas para la cuenta conectada
- `aws cur --region aws_region delete-report-definition --report-name report` : elimina una definición de informe de uso

## Links
- More information: https://docs.aws.amazon.com/cli/latest/reference/cur/
