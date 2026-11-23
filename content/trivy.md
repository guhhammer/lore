# trivy

Scanner for vulnerabilities in container images, filesystems, and Git repositories, as well as for configuration issues.

## Usage
- `trivy image image:tag` : scan a Docker image for vulnerabilities and exposed secrets
- `trivy image -s HIGH,CRITICAL alpine:3.15` : scan a Docker image filtering the output by severity
- `trivy image --ignore-unfixed alpine:3.15` : scan a Docker image ignoring any unfixed/unpatched vulnerabilities
- `trivy fs --scanners vuln,misconfig path/to/project_directory` : scan the filesystem for vulnerabilities and misconfigurations
- `trivy config path/to/iac_directory` : scan an IaC (Terraform, CloudFormation, ARM, Helm, and Dockerfile) directory for misconfigurations
- `trivy repo path/to/local_repository_directory|remote_repository_url` : scan a local or remote Git repository for vulnerabilities
- `trivy repo --commit commit_hash repository` : scan a Git repository up to a specific commit hash
- `trivy image -f template -t "@sarif.tpl" -o path/to/report.sarif image:tag` : generate output with a SARIF template

## Categories
containers, media, version-control

## Links
- More information: https://trivy.dev/docs/latest/guide/references/configuration/cli/trivy/
- tldr page: https://tldr.inbrowser.app/pages/common/trivy
