# mailx

Send and receive mail.

## Usage
- `mailx -s "subject" to_addr` : send mail (the content should be typed after the command, and ended with `<Ctrl d>`)
- `echo "content" | mailx -s "subject" to_addr` : send mail with content passed from another command
- `mailx <content.txt -s "subject" to_addr` : send mail with content read from a file
- `mailx -s "subject" -c cc_addr to_addr` : send mail to a recipient and CC to another address
- `mailx -s "subject" -r from_addr to_addr` : send mail specifying the sender address
- `mailx -a path/to/file -s "subject" to_addr` : send mail with an attachment

## Categories
files

## Links
- More information: https://manned.org/mailx
- tldr page: https://tldr.inbrowser.app/pages/common/mailx
