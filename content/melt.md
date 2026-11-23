# melt

Back up and restore Ed25519 SSH keys using memorizable seed phrases.

## Usage
- `melt ~/.ssh/id_ed25519` : generate a seed phrase from an existing Ed25519 private key
- `cat ~/.ssh/id_ed25519 | melt` : generate a seed phrase from `stdin`
- `melt restore path/to/key --seed "seed_phrase"` : restore an SSH key from a seed phrase
- `cat path/to/file | melt restore -` : restore an SSH key from a seed phrase via `stdin`

## Categories
network, files

## Links
- More information: https://github.com/charmbracelet/melt#usage
- tldr page: https://tldr.inbrowser.app/pages/common/melt
