# pass

pass is a password manager made of gpg and plain files: one encrypted file per password, in a folder you can put under git.

## Tasks
### Start
- `gpg --list-secret-keys` : find the key ID or email you will use
- `pass init KEYID` : make the store, encrypting with that key
- `pass git init` : keep the store under git
- `pass` : list every entry as a tree
### Add
- `pass insert site/example` : add a password by typing it
- `pass insert -m site/example` : add several lines (password, then notes)
- `pass generate site/example 20` : make a random 20-character password
- `pass generate -n site/example 20` : the same without symbols
- `pass edit site/example` : change an entry in your editor
### Read
- `pass site/example` : print the entry
- `pass -c site/example` : copy the first line to the clipboard for 45 seconds
- `pass show site/example | head -n 1` : just the password line
- `pass find example` : search entry names
- `pass grep TEXT` : search inside decrypted entries
### Organize
- `pass mv site/old site/new` : rename an entry
- `pass cp site/a site/b` : copy an entry
- `pass rm site/example` : delete an entry (deletes it)
- `pass rm -r site` : delete a whole folder (deletes it)
### Sync and share
- `pass git remote add origin URL` : set the remote for the store
- `pass git push -u origin main` : send the store to the remote
- `pass git pull` : bring changes from another machine
- `pass init KEY1 KEY2` : encrypt for several keys, to share a store
- `PASSWORD_STORE_DIR=~/work-pass pass init KEYID` : keep a separate store

## Notes
- File names and folders are not encrypted, only the content: do not hide secrets in names.
- The store lives in `~/.password-store`; back it up together with your gpg private key.
- Losing the gpg key means losing every password: see the gpg page for backups.
- Browser add-ons and phone apps (passff, Android Password Store) can read the same store.

## See also
- gpg
- git
- ssh-keys
- openssl

## Links
- Home: https://www.passwordstore.org
- Manual: https://git.zx2c4.com/password-store/about/

## Categories
security
