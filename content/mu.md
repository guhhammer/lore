# mu

Index and search emails from a local Maildir.

## Usage
- `mu init --maildir=path/to/directory --my-address=name@example.com` : initialize the email database, optionally specifying the Maildir directory and email addresses
- `mu index` : index new emails
- `mu find keyword` : find messages using a specific keyword (in message body, subject, sender, ...)
- `mu find to:alice subject:jellyfish apples OR oranges` : find messages to Alice with subject `jellyfish` containing the words `apples` or `oranges`
- `mu find 'subject:soc*' flag:unread maildir:'/Sent Items'` : find unread messages about words starting with `soc` (the `*` only works at the end of the search term) in the Sent Items folder
- `mu find 'mime:image/*' size:2k..2m date:20210101..20211231 from:sam` : find messages from Sam with attached images, between 2 KiB and 2 MiB, written in 2021
- `mu cfind Bob` : list contacts with `Bob` in either name or email address

## Categories
files

## Links
- More information: https://man.cx/mu
- tldr page: https://tldr.inbrowser.app/pages/common/mu
