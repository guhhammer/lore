# k8sec

Manage Kubernetes secrets.

## Usage
- `k8sec list` : list all secrets
- `k8sec list secret_name --base64` : list a specific secret as a Base64-encoded string
- `k8sec set secret_name key=value` : set a secret's value
- `k8sec set --base64 secret_name key=encoded_value` : set a Base64-encoded value
- `k8sec unset secret_name` : unset a secret
- `k8sec load -f path/to/file secret_name` : load secrets from a file
- `k8sec dump -f path/to/file secret_name` : dump secrets to a file

## Categories
files, containers

## Links
- More information: https://github.com/dtan4/k8sec
- tldr page: https://tldr.inbrowser.app/pages/common/k8sec
