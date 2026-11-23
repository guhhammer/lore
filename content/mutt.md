# mutt

Command-line email client.

## Usage
- `mutt -f mailbox` : open the specified mailbox
- `mutt -s subject -c cc@example.com recipient@example.com` : send an email and specify a subject and a cc recipient
- `mutt -a file1 file2 ... -- recipient@example.com` : send an email with files attached
- `mutt -i path/to/file recipient@example.com` : specify a file to include as the message body
- `mutt -H path/to/file recipient@example.com` : specify a draft file containing the header and the body of the message, in RFC 5322 format

## Categories
files, text

## Links
- tldr page: https://tldr.inbrowser.app/pages/common/mutt
