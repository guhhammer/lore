# ykman config

In- of uitschakelen van YubiKey-applicaties. Opmerking: je kan `ykman info` gebruiken om de huidige ingeschakelde applicaties te zien.

## Usage
- `ykman config usb|nfc -e otp|u2f|fido2|oath|piv|openpgp|hsmauth` : schakel een applicatie in via USB of NFC (`--enable` kan meerdere keren gebruikt worden om meerdere applicaties te specificeren)
- `ykman config usb|nfc -d otp|u2f|fido2|oath|piv|openpgp|hsmauth` : schakel een applicatie uit via USB of NFC (`--disable` kan meerdere keren gebruikt worden om meerdere applicaties te specificeren)
- `ykman config nfc -D` : schakel alle applicaties uit via NFC
