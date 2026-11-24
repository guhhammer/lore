# pass otp

A pass extension for managing one-time-password (OTP) tokens.

## Usage
- `pass otp insert path/to/pass` : prompt for an otpauth URI token and create a new pass file
- `pass otp append path/to/pass` : prompt for an otpauth URI token and append to an existing pass file
- `pass otp path/to/pass` : print a 2FA code using the OTP token in a pass file
- `pass otp -c path/to/pass` : copy and don't print a 2FA code using the OTP token in a pass file
- `pass otp uri -q path/to/pass` : display a QR code using the OTP token stored in a pass file
- `pass otp append -s -i issuer_name -a account_name path/to/pass` : prompt for an OTP secret value specifying issuer and account (at least one must be specified) and append to existing pass file

## Categories
files, users, shell

## Links
- More information: https://manned.org/pass-otp
- tldr page: https://tldr.inbrowser.app/pages/common/pass-otp
- Wikipedia: https://en.wikipedia.org/wiki/Pass_(software)
