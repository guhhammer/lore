# aws codeartifact

Gerencia repositórios, domínios, pacotes, versões de pacotes e ativos do CodeArtifact. O CodeArtifact é um repositório de artefatos compatível com gerenciadores de pacotes populares e ferramentas de construção como Maven, Gradle, npm, Yarn, Twine, pip, NuGet e SwiftPM.

## Usage
- `aws codeartifact list-domains` : lista domínios disponíveis para a sua conta da AWS
- `aws codeartifact login --tool npm|pip|twine --domain seu_domínio --repository nome_do_repositório` : gera credenciais para um gerenciador de pacote específico
- `aws codeartifact get-repository-endpoint --domain seu_domínio --repository nome_do_repositório --format npm|pypi|maven|nuget|generic` : recupera a URL do endpoint de um repositório do CodeArtifact
- `aws codeartifact help` : mostra ajuda
- `aws codeartifact subcomando help` : exibe ajuda para um subcomando específico

## Links
- More information: https://docs.aws.amazon.com/cli/latest/reference/codeartifact/
