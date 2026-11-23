# gcloud kms decrypt

Decrypt a ciphertext file using a Cloud KMS key.

## Usage
- `gcloud kms decrypt --key=key_name --keyring=keyring_name --location=global --ciphertext-file=path/to/ciphertext --plaintext-file=path/to/plaintext` : decrypt a file using a specified key, key ring, and location
- `gcloud kms decrypt --key=key_name --keyring=keyring_name --location=global --additional-authenticated-data-file=path/to/file.aad --ciphertext-file=path/to/ciphertext --plaintext-file=-` : decrypt a file with additional authenticated data (AAD) and write the decrypted plaintext to `stdout`

## Categories
files, security, cloud

## Links
- More information: https://docs.cloud.google.com/sdk/gcloud/reference/kms/decrypt
- tldr page: https://tldr.inbrowser.app/pages/common/gcloud-kms-decrypt
