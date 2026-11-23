# ansible-vault

Cifra e decifra valori, strutture dati e file nei progetti Ansible.

## Usage
- `ansible-vault create percorso/al/vault_file` : crea un nuovo file vault crittografato con richiesta interattiva della password
- `ansible-vault edit|view|rekey percorso/al/vault_file` : modifica, visualizza o ricifra (re-encrypt) un file vault crittografato esistente con richiesta della password
- `ansible-vault create --vault-password-file percorso/al/password_file percorso/al/vault_file` : crea un nuovo file vault crittografato usando un file password per la cifratura
- `ansible-vault encrypt --vault-password-file percorso/al/password_file percorso/del/file` : cifra un file plaintext esistente sul posto usando un file password
- `ansible-vault encrypt_string` : cifra una stringa usando il formato stringa crittografata di Ansible con prompt interattivi
- `ansible-vault view --vault-password-file percorso/al/password_file percorso/al/vault_file` : visualizza un file vault crittografato usando un file password per la decifratura
- `ansible-vault rekey --vault-password-file percorso/al/vecchio_password_file --new-vault-password-file percorso/al/nuovo_password_file percorso/al/vault_file` : ricifra (re-encrypt) un file vault già crittografato con un nuovo file password

## Links
- More information: https://docs.ansible.com/projects/ansible/latest/vault_guide/index.html
