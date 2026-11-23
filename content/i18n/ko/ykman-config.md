# ykman config

YubiKey 애플리케이션 활성화 또는 비활성화. 참고: 현재 활성화된 애플리케이션을 보려면 `ykman info`를 사용할 수 있습니다.

## Usage
- `ykman config usb|nfc -e otp|u2f|fido2|oath|piv|openpgp|hsmauth` : USB 또는 NFC를 통해 애플리케이션 활성화 (`--enable`을 여러 번 사용하여 더 많은 애플리케이션을 지정할 수 있음)
- `ykman config usb|nfc -d otp|u2f|fido2|oath|piv|openpgp|hsmauth` : USB 또는 NFC를 통해 애플리케이션 비활성화 (`--disable`을 여러 번 사용하여 더 많은 애플리케이션을 지정할 수 있음)
- `ykman config nfc -D` : NFC를 통해 모든 애플리케이션 비활성화
