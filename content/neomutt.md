# neomutt

NeoMutt email client.

## Usage
- `neomutt -f path/to/mailbox` : open the specified mailbox
- `neomutt -s "subject" -c cc@example.com recipient@example.com` : start writing an email and specify a subject and a `cc` recipient
- `neomutt -a path/to/file1 path/to/file2 ... -- recipient@example.com` : send an email with files attached
- `neomutt -i path/to/file recipient@example.com` : specify a file to include as the message body
- `neomutt -H path/to/file recipient@example.com` : specify a draft file containing the header and the body of the message, in RFC 5322 format

## Categories
files

## Links
- More information: https://neomutt.org/guide/reference.html
- tldr page: https://tldr.inbrowser.app/pages/common/neomutt
