# notmuch

Index, search, read, and tag large collections of email messages.

## Usage
- `notmuch setup` : configure for first use
- `notmuch tag +custom_tag "search_term"` : add a tag for all messages matching a search term
- `notmuch tag -custom_tag "search_term"` : remove a tag for all messages matching a search term
- `notmuch count --output=messages|threads "search_term"` : count messages matching the given search term
- `notmuch search --format=json|text --output=summary|threads|messages|files|tags "search_term"` : search for messages matching the given search term
- `notmuch search --format=json|text --output=summary|threads|messages|files|tags --limit=X "search_term"` : limit the number of search results to X
- `notmuch reply --format=default|headers-only --reply-to=sender|all "search_term"` : create a reply template for a set of messages

## Categories
text

## Links
- More information: https://notmuchmail.org/manpages/
- tldr page: https://tldr.inbrowser.app/pages/common/notmuch
