# wacli

WhatsApp command-line client for sending messages, managing chats, and contacts. Some subcommands such as `auth`, `sync`, etc. have their own usage documentation.

## Usage
- `wacli auth` : authenticate via QR code
- `wacli sync --once` : synchronize messages once and exit
- `wacli send text --to phone_number --message "message"` : send a text message to a phone number
- `wacli messages list --chat phone_number@s.whatsapp.net --json` : list messages from a specific chat
- `wacli chats list --json` : list all chats
- `wacli contacts search "name" --json` : search contacts by name
- `wacli doctor` : run diagnostics on the store
- `wacli version` : display version

## Categories
text, documents, monitoring

## Links
- More information: https://wacli.sh/
- tldr page: https://tldr.inbrowser.app/pages/common/wacli
