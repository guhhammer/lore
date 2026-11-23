# wacli send text

Send a text message via WhatsApp.

## Usage
- `wacli send text --to phone_number --message "message"` : send a text message to a phone number
- `wacli send text --to phone_number@s.whatsapp.net --message "message"` : send a text message to a specific JID
- `wacli send text --to phone_number --message "message" --timeout 10m` : send a text message with a custom timeout
- `wacli send text --to phone_number --message "message" --store path/to/store` : send a text message using a custom store path

## See also
- wacli send file

## Categories
text

## Links
- More information: https://wacli.sh/send.html
- tldr page: https://tldr.inbrowser.app/pages/common/wacli-send-text
