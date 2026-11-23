# wacli messages list

List messages from a WhatsApp chat.

## Usage
- `wacli messages list --chat phone_number@s.whatsapp.net --json` : list messages from a specific chat in JSON format
- `wacli messages list --chat phone_number@s.whatsapp.net --after 2026-01-28 --json` : list messages received after a specific date
- `wacli messages list --chat phone_number@s.whatsapp.net --before 2026-01-28 --json` : list messages received before a specific date
- `wacli messages list --chat phone_number@s.whatsapp.net --after 2026-01-01 --before 2026-01-31 --json` : list messages within a date range
- `wacli messages list --chat phone_number@s.whatsapp.net --limit 10 --json` : limit the number of results

## See also
- wacli messages search
- wacli messages show
- wacli messages context

## Categories
text

## Links
- More information: https://wacli.sh/messages.html
- tldr page: https://tldr.inbrowser.app/pages/common/wacli-messages-list
