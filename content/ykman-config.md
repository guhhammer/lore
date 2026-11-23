# ykman config

Enable or disable YubiKey applications. Note: You can use `ykman info` to see currently enabled applications.

## Usage
- `ykman config usb|nfc -e otp|u2f|fido2|oath|piv|openpgp|hsmauth` : enable an application over USB or NFC (`--enable` can be used multiple times to specify more applications)
- `ykman config usb|nfc -d otp|u2f|fido2|oath|piv|openpgp|hsmauth` : disable an application over USB or NFC (`--disable` can be used multiple times to specify more applications)
- `ykman config nfc -D` : disable all applications over NFC

## Categories
hardware

## Links
- tldr page: https://tldr.inbrowser.app/pages/common/ykman-config
