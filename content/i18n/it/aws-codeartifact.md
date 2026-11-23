# aws codeartifact

Gestisce repository CodeArtifact, domini, pacchetti, versioni di pacchetti e asset. CodeArtifact è un repository di artefatti compatibile con gestori di pacchetti e strumenti di build popolari come Maven, Gradle, npm, Yarn, Twine, pip, NuGet e SwiftPM.

## Usage
- `aws codeartifact list-domains` : elenca i domini disponibili per il tuo account AWS
- `aws codeartifact login --tool npm|pip|twine --domain tuo_dominio --repository nome_repository` : genera credenziali per un gestore di pacchetti specifico
- `aws codeartifact get-repository-endpoint --domain tuo_dominio --repository nome_repository --format npm|pypi|maven|nuget|generic` : ottieni l'URL endpoint di un repository CodeArtifact
- `aws codeartifact help` : mostra l'aiuto
- `aws codeartifact sottocomando help` : visualizza l'aiuto per un sottocomando specifico

## Links
- More information: https://docs.aws.amazon.com/cli/latest/reference/codeartifact/
