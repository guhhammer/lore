# vault

Interact with HashiCorp Vault.

## Usage
- `vault init` : connect to a Vault server and initialize a new encrypted data store
- `vault unseal key-share-x` : unseal (unlock) the vault, by providing one of the key shares needed to access the encrypted data store
- `vault auth authentication_token` : authenticate the CLI client against the Vault server, using an authentication token
- `vault write secret/hello value=world` : store a new secret in the vault, using the generic back-end called "secret"
- `vault read secret/hello` : read a value from the vault, using the generic back-end called "secret"
- `vault read -field=field_name secret/hello` : read a specific field from the value
- `vault seal` : seal (lock) the Vault server, by removing the encryption key of the data store from memory

## Links
- More information: https://developer.hashicorp.com/vault/docs/commands
- tldr page: https://tldr.inbrowser.app/pages/common/vault
