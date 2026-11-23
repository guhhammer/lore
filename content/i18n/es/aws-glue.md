# aws glue

CLI para AWS Glue. Define el punto de enlace público para el servicio AWS Glue.

## Usage
- `aws glue list-jobs` : lista trabajos
- `aws glue start-job-run --job-name nombre_del_trabajo` : inicia un trabajo
- `aws glue start-workflow-run --name nombre_del_flujo` : inicia la ejecución de un flujo de trabajo
- `aws glue list-triggers` : lista disparadores
- `aws glue start-trigger --name nombre_disparador` : inicia un disparador
- `aws glue create-dev-endpoint --endpoint-name nombre --role-arn role_arn_usado_por_puntofinal` : crea un punto final de desarrollo

## Links
- More information: https://docs.aws.amazon.com/cli/latest/reference/glue/
