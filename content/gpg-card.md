# gpg-card

Administrate OpenPGP and PIV smart cards. Similar to `gpg --card-edit`.

## Usage
- `gpg-card` : start in interactive mode
- `gpg-card command1 -- command2 -- command3` : invoke one or more commands non-interactively
- `gpg-card list` : show information about a smart card
- `gpg-card fetch` : retrieve the public key using the URL stored on an OpenPGP card
- `gpg-card url` : set the URL used by the `fetch` command
- `gpg-card passwd` : change or unblock PINs (uses the default action for the card in non-interactive mode)
- `gpg-card forcesig` : toggle the forcesig flag of an OpenPGP card (i.e. require entering the user PIN for signing)
- `gpg-card factory-reset` : factory reset a smart card (i.e. delete all data and reset PINs)

## Categories
security, network

## Links
- More information: https://manned.org/gpg-card
- tldr page: https://tldr.inbrowser.app/pages/common/gpg-card
