# tgutil

Manage a Telegram account.

## Usage
- `tgutil -n session_name -u chat_id -m edit --text "current_text" --newtext "new_text"` : edit the last text message on a chat with new text
- `tgutil -n session_name -u chat_id -m editall --text "current_text" --newtext "new_text"` : edit all text messages that contain `current_text` with `new_text`
- `tgutil -n session_name -u chat_id -m delete --text "current_text"` : delete the last text message on a chat with search value
- `tgutil -n session_name -u chat_id -m deleteall --text "query"` : delete all messages containing the `query` value

## See also
- tgcloud
- tgsend
- tginfo

## Categories
text, users

## Links
- More information: https://pypi.org/project/telegram-cloud/
- tldr page: https://tldr.inbrowser.app/pages/common/tgutil
