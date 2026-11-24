# mail

Operate on the user's mailbox. To send an email the message body is built from `stdin`.

## Usage
- `mail` : open an interactive prompt to check personal mail
- `mail --subject "subject_line" to_user@example.com --cc "cc_email_address"` : send a typed email message with optional CC. The command-line below continues after pressing `<Enter>`. Input message text (can be multiline). Press `<Ctrl d>` to complete the message text
- `mail <path/to/file.txt --subject "$HOSTNAME filename.txt" to_user@example.com` : send an email that contains file content
- `tar cvzf - path/to/directory1 path/to/directory2 | uuencode data.tar.gz | mail --subject "subject_line" to_user@example.com` : send a `.tar.gz` file as an attachment
- `mail -h` : display help

## Categories
files, archives, text

## Links
- More information: https://manned.org/mail
- tldr page: https://tldr.inbrowser.app/pages/common/mail
- Wikipedia: https://en.wikipedia.org/wiki/Mail_(Unix)
