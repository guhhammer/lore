# prowler

Perform security best practices assessments, audits, and compliance checks across AWS, Azure, Google Cloud, and Kubernetes. Some subcommands such as  `azure`, `gcp`, `kubernetes`, etc. have their own usage documentation.

## Usage
- `prowler provider` : run an AWS, Azure, GCP, Kubernetes - as provider - audit with default checks
- `prowler provider -l` : show all available checks for a specific provider
- `prowler provider --list-services` : show all available services for a specific provider
- `prowler provider --output-modes csv,json-asff,html,...` : generate output in multiple formats, including JSON-ASFF for AWS Security Hub
- `prowler provider --verbose` : execute in verbose mode
- `prowler provider --status PASS,FAIL,MANUAL` : filter findings by status
- `prowler --help` : display help
- `prowler -v` : display version

## Categories
cloud, containers, documents

## Links
- More information: https://docs.prowler.com/user-guide/cli/tutorials/misc
- tldr page: https://tldr.inbrowser.app/pages/common/prowler
