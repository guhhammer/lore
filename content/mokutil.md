# mokutil

Configure Secure Boot Machine Owner Keys (MOK). Some operations, such as enabling and disabling Secure Boot or enrolling keys require a reboot.

## Usage
- `mokutil --sb-state` : show if Secure Boot is enabled
- `mokutil --enable-validation` : enable Secure Boot
- `mokutil --disable-validation` : disable Secure Boot
- `mokutil -l` : list enrolled keys
- `mokutil -i path/to/key.der` : enroll a new key
- `mokutil -N` : list the keys to be enrolled
- `mokutil --set-verbosity true` : set shim verbosity

## Categories
system, security

## Links
- More information: https://manned.org/mokutil
- tldr page: https://tldr.inbrowser.app/pages/linux/mokutil
