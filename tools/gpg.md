# gpg

gpg (GnuPG) encrypts, decrypts and signs files and messages with keys. It also checks signatures on downloads and git tags.

## Tasks
### Make and look at keys
- `gpg --full-generate-key` : make a key pair, answering the questions
- `gpg --list-keys` : public keys on your keyring
- `gpg --list-secret-keys --keyid-format long` : your own keys with their long IDs
- `gpg --fingerprint EMAIL` : a key's fingerprint
- `gpg --edit-key KEYID` : change expiry, add a user id (type help)
### Share keys
- `gpg --armor --export EMAIL > public.asc` : export your public key as text
- `gpg --import FILE.asc` : add someone's key
- `gpg --keyserver keys.openpgp.org --send-keys KEYID` : publish your key
- `gpg --keyserver keys.openpgp.org --recv-keys KEYID` : fetch a key
- `gpg --armor --export-secret-keys KEYID > secret.asc` : export the private key for a backup (keep it safe)
### Encrypt and decrypt
- `gpg --encrypt --recipient EMAIL FILE` : encrypt for someone, writes FILE.gpg
- `gpg --armor --encrypt --recipient EMAIL FILE` : the same as text, FILE.asc
- `gpg --decrypt FILE.gpg > FILE` : decrypt to a file
- `gpg --symmetric FILE` : encrypt with just a passphrase
- `gpg --output FILE --decrypt FILE.gpg` : decrypt, choosing the output name
### Sign and verify
- `gpg --detach-sign --armor FILE` : make a signature file FILE.asc
- `gpg --verify FILE.asc FILE` : check a file against its signature
- `gpg --clearsign FILE` : sign text so it stays readable
- `gpg --verify SHA256SUMS.sig SHA256SUMS` : verify a checksum list from a download page
### Use it with git and delete keys
- `git config --global user.signingkey KEYID` : tell git which key signs
- `git config --global commit.gpgsign true` : sign every commit
- `gpg --delete-secret-and-public-key KEYID` : remove a key pair (deletes it)
- `gpg --gen-revoke KEYID > revoke.asc` : make a revocation certificate in advance

## Notes
- Back up your private key and the revocation certificate somewhere safe: a lost key cannot be recovered.
- Anyone can publish a key under your email: trust a key by checking its fingerprint another way.
- `export GPG_TTY=$(tty)` in your shell startup fixes passphrase prompts that do not appear.
- Short key IDs can be forged; compare full fingerprints.

## See also
- pass
- openssl
- ssh-keys
- age

## Links
- Manual: https://www.gnupg.org/documentation/manuals/gnupg/
- Guides: https://www.gnupg.org/documentation/guides.html

## Categories
security
