# ykman openpgp

Manage the OpenPGP YubiKey application. Note: You need to use `gpg --card-edit` for some settings.

## Usage
- `ykman openpgp info` : display general information about the OpenPGP application
- `ykman openpgp access set-retries 3 3 3` : set the number of retry attempts for the User PIN, Reset Code, and Admin PIN, respectively
- `ykman openpgp access change-pin|reset-code|admin-pin` : change the User PIN, Reset Code, or Admin PIN
- `ykman openpgp reset` : factory reset the OpenPGP application (you have to do this after exceeding the number of Admin PIN retry attempts)

## Categories
security

## Links
- More information: https://docs.yubico.com/software/yubikey/tools/ykman/OpenPGP_Commands.html
- tldr page: https://tldr.inbrowser.app/pages/common/ykman-openpgp
