# aws accessanalyzer

Analiza y revisa las políticas de recursos para identificar posibles riesgos de seguridad.

## Usage
- `aws accessanalyzer create-analyzer --analyzer-name nombre_analizador --type tipo --tags etiquetas` : crea un nuevo Access Analyzer
- `aws accessanalyzer delete-analyzer --analyzer-arn analizador_arn` : elimina un analizador de acceso existente
- `aws accessanalyzer get-analyzer --analyzer-arn analizador_arn` : obtiene detalles de un analizador de acceso específico
- `aws accessanalyzer list-analyzers` : lista todos los analizadores de acceso
- `aws accessanalyzer update-analyzer --analyzer-arn analizador_arn --tags nuevas_etiquetas` : actualiza la configuración de un analizador de acceso
- `aws accessanalyzer create-archive-rule --analyzer-arn analizador_arn --rule-name nombre_regla --filter filtro` : crea una nueva regla de archivo de Access Analyzer
- `aws accessanalyzer delete-archive-rule --analyzer-arn analizador_arn --rule-name nombre_regla` : elimina una regla de archivo de Access Analyzer
- `aws accessanalyzer list-archive-rules --analyzer-arn analizador_arn` : lista todas las reglas de archivo de Access Analyzer

## Links
- More information: https://docs.aws.amazon.com/cli/latest/reference/accessanalyzer/
