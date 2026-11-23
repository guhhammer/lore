# swaks

Swiss Army Knife SMTP, the all-purpose SMTP transaction tester.

## Usage
- `swaks -t user@example.com -s test-server.example.net` : deliver a standard test email to `user@example.com` on port 25 of `test-server.example.net`
- `swaks -t user@example.com -f me@example.com -a CRAM-MD5 -au me@example.com --header-X-Test "test_email"` : deliver a standard test email, requiring CRAM-MD5 authentication as user `me@example.com`. An "X-Test" header will be added to the email body
- `swaks -t user@example.com --attach - -s test-server.example.com -n path/to/eicar.txt` : test a virus scanner using EICAR in an attachment. Don't show the message DATA part
- `swaks -t user@example.com --body path/to/gtube_file` : test a spam scanner using GTUBE in the body of an email, routed via the MX records for `example.com`
- `swaks -t user@example.com --socket /var/lda.sock --protocol LMTP` : deliver a standard test email to `user@example.com` using the LMTP protocol via a UNIX domain socket file

## Categories
development, users, network

## Links
- More information: https://github.com/jetmore/swaks/blob/develop/doc/base.pod
- tldr page: https://tldr.inbrowser.app/pages/linux/swaks
