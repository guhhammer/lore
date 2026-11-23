# nms

Tool that recreates the famous data decryption effect seen in the 1992 movie Sneakers from `stdin`.

## Usage
- `echo "Hello, World!" | nms` : decrypt text after a keystroke
- `ls -la | nms -a` : decrypt output immediately, without waiting for a keystroke
- `cat path/to/file | nms -a -f blue|white|yellow|black|magenta|green|red` : decrypt the content of a file, with a custom output color
- `command | nms -a -c` : clear the screen before decrypting

## Categories
files, security

## Links
- More information: https://github.com/bartobri/no-more-secrets
- tldr page: https://tldr.inbrowser.app/pages/common/nms
