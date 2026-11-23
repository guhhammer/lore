# kubectl certificate

Manage certificate signing requests.

## Usage
- `kubectl certificate approve csr_name` : approve a certificate signing request by name
- `kubectl certificate deny csr_name` : deny a certificate signing request by name
- `kubectl certificate approve --filename path/to/csr.yaml` : approve a certificate signing request defined in a manifest file
- `kubectl certificate deny --filename path/to/csr.yaml` : deny a certificate signing request defined in a manifest file
- `kubectl certificate approve --force csr_name` : force reapproval of a certificate signing request that was previously denied

## Categories
security, files

## Links
- More information: https://kubernetes.io/docs/reference/kubectl/generated/kubectl_certificate/
- tldr page: https://tldr.inbrowser.app/pages/common/kubectl-certificate
