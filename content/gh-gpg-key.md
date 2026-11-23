# gh gpg-key

Manage GPG keys registered with the authorized GitHub account.

## Usage
- `gh gpg-key ls` : list GPG keys in the authorized GitHub account
- `gh gpg-key add path/to/key_file` : add a GPG key to the authorized GitHub account by specifying the key file
- `gpg -a --export key_id | gh gpg-key add -` : add a GPG key to the authorized GitHub account by specifying the key ID
- `gh gpg-key delete key_id` : delete a GPG key from the authorized GitHub account

## See also
- gpg

## Categories
security, users

## Links
- More information: https://cli.github.com/manual/gh_gpg-key
- tldr page: https://tldr.inbrowser.app/pages/common/gh-gpg-key
