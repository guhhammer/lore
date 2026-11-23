# kubectl replace

Replace a resource by file or `stdin`.

## Usage
- `kubectl replace -f path/to/file.yml` : replace the resource using the resource definition file
- `kubectl replace -f -` : replace the resource using the input passed into `stdin`
- `kubectl replace -f path/to/file.yml --force` : force replace, delete, and then re-create the resource

## Categories
files

## Links
- More information: https://kubernetes.io/docs/reference/kubectl/generated/kubectl_replace/
- tldr page: https://tldr.inbrowser.app/pages/common/kubectl-replace
