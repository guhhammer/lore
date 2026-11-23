# aws accessanalyzer

Analizza e rivedi le policy delle risorse per identificare potenziali rischi per la sicurezza.

## Usage
- `aws accessanalyzer create-analyzer --analyzer-name nome_analizzatore --type tipo --tags tag` : crea un nuovo Access Analyzer
- `aws accessanalyzer delete-analyzer --analyzer-arn arn_analizzatore` : elimina un Access Analyzer esistente
- `aws accessanalyzer get-analyzer --analyzer-arn arn_analizzatore` : ottiene i dettagli di un Access Analyzer specifico
- `aws accessanalyzer list-analyzers` : elenca tutti gli Access Analyzer
- `aws accessanalyzer update-analyzer --analyzer-arn arn_analizzatore --tags nuovi_tag` : aggiorna le impostazioni di un Access Analyzer
- `aws accessanalyzer create-archive-rule --analyzer-arn arn_analizzatore --rule-name nome_regola --filter filtro` : crea una nuova regola di archiviazione per un Access Analyzer
- `aws accessanalyzer delete-archive-rule --analyzer-arn arn_analizzatore --rule-name nome_regola` : elimina una regola di archiviazione di un Access Analyzer
- `aws accessanalyzer list-archive-rules --analyzer-arn arn_analizzatore` : elenca tutte le regole di archiviazione di un Access Analyzer

## Links
- More information: https://docs.aws.amazon.com/cli/latest/reference/accessanalyzer/
