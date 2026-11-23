# exiqgrep

Perl script offering possibilities to `grep` in the Exim queue output.

## Usage
- `exiqgrep -f '<email@example.com>'` : match the sender address using a case-insensitive search
- `exiqgrep -i -f '<email@example.com>'` : match the sender address and display message IDs only
- `exiqgrep -r 'email@example.com'` : match the recipient address
- `exiqgrep -i -f '<email@example.com>' | xargs exim -Mrm` : remove all messages matching the sender address from the queue
- `exiqgrep -f '^<>$'` : test for bounced messages
- `exiqgrep -c -f '^<>$'` : display the count of bounced messages

## Categories
development

## Links
- tldr page: https://tldr.inbrowser.app/pages/linux/exiqgrep
