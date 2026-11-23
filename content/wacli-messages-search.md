# wacli messages search

Search WhatsApp messages using full-text search.

## Usage
- `wacli messages search "query" --chat phone_number@s.whatsapp.net --json` : search messages in a specific chat
- `wacli messages search "query" --from phone_number@s.whatsapp.net --json` : search messages from a specific sender
- `wacli messages search "query" --chat phone_number@s.whatsapp.net --after 2026-01-28 --json` : search messages after a specific date
- `wacli messages search "query" --type image|video|audio|document --json` : search messages of a specific type
- `wacli messages search "query" --chat phone_number@s.whatsapp.net --limit 10 --json` : search messages with a result limit

## See also
- wacli messages list
- wacli messages show

## Categories
text, databases, media

## Links
- More information: https://wacli.sh/messages.html
- tldr page: https://tldr.inbrowser.app/pages/common/wacli-messages-search
