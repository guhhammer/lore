# ansible-vault

Encrypt and decrypt values, data structures, and files within Ansible projects.

## Usage
- `ansible-vault create path/to/vault_file` : create a new encrypted vault file with a prompt for a password
- `ansible-vault edit|view|rekey path/to/vault_file` : edit, view, or re-key (re-encrypt) an existing encrypted vault file with a prompt for the password
- `ansible-vault create --vault-password-file path/to/password_file path/to/vault_file` : create a new encrypted vault file using a password file to encrypt it
- `ansible-vault encrypt --vault-password-file path/to/password_file path/to/file` : encrypt an existing plaintext file in-place using a password file
- `ansible-vault encrypt_string` : encrypt a string using Ansible's encrypted string format, displaying interactive prompts
- `ansible-vault view --vault-password-file path/to/password_file path/to/vault_file` : view an encrypted vault file by using a password file to decrypt
- `ansible-vault rekey --vault-password-file path/to/old_password_file --new-vault-password-file path/to/new_password_file path/to/vault_file` : re-key (re-encrypt) an already encrypted vault file with a new password file

## Categories
files, security, users

## Links
- More information: https://docs.ansible.com/projects/ansible/latest/vault_guide/index.html
- tldr page: https://tldr.inbrowser.app/pages/common/ansible-vault
