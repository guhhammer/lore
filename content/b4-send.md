# b4 send

Send a patch series prepared with `b4 prep` to a mailing list.

## Usage
- `b4 send` : send patches from the current prep-tracked branch
- `b4 send -o path/to/directory` : generate the messages to review without sending them
- `b4 send --preview-to email_address1 email_address2 ...` : send patches to specific recipients for preview without rerolling the series version
- `b4 send --resend version_number` : send a previously sent version of the series again
- `b4 send -d` : simulate sending the path series without sending any mail

## Categories
version-control, files

## Links
- More information: https://b4.docs.kernel.org/en/latest/contributor/send.html
- tldr page: https://tldr.inbrowser.app/pages/linux/b4-send
