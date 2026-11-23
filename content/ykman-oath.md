# ykman oath

Manage the OATH YubiKey application. A `keyword` can be a part of the name or the issuer.

## Usage
- `ykman oath info` : display general information about the OATH application
- `ykman oath access change` : change the password used to protect OATH accounts (add `--clear` to remove it)
- `ykman oath accounts add -i issuer name` : add a new account (the issuer is optional)
- `ykman oath accounts list` : list all accounts (with their issuers)
- `ykman oath accounts code keyword` : list all accounts with their current TOTP/HOTP codes (optionally filtering the list with a keyword)
- `ykman oath accounts rename keyword issuer:name|name` : rename an account
- `ykman oath accounts delete keyword` : delete an account
- `ykman oath reset` : delete all accounts and restore factory settings

## Categories
users

## Links
- More information: https://docs.yubico.com/software/yubikey/tools/ykman/OATH_Commands.html
- tldr page: https://tldr.inbrowser.app/pages/common/ykman-oath
