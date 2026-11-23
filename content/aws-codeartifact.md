# aws codeartifact

Manage CodeArtifact repositories, domains, packages, package versions, and assets. CodeArtifact is an artifact repository compatible with popular package managers and build tools like Maven, Gradle, npm, Yarn, Twine, pip, NuGet, and SwiftPM.

## Usage
- `aws codeartifact list-domains` : list available domains for your AWS account
- `aws codeartifact login --tool npm|pip|twine --domain your_domain --repository repository_name` : generate credentials for a specific package manager
- `aws codeartifact get-repository-endpoint --domain your_domain --repository repository_name --format npm|pypi|maven|nuget|generic` : get the endpoint URL of a CodeArtifact repository
- `aws codeartifact help` : display help
- `aws codeartifact subcommand help` : display help for a specific subcommand

## Categories
packages, cloud, development

## Links
- More information: https://docs.aws.amazon.com/cli/latest/reference/codeartifact/
- tldr page: https://tldr.inbrowser.app/pages/common/aws-codeartifact
