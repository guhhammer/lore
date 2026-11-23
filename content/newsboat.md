# newsboat

An RSS/Atom feed reader for text terminals.

## Usage
- `newsboat -i my-feeds.xml` : first import feed URLs from an OPML file
- `echo http://example.com/path/to/feed >>"${HOME}/.newsboat/urls"` : alternatively, add feeds manually
- `newsboat -r` : start Newsboat and refresh all feeds on startup
- `newsboat -x reload|print-unread|...` : execute one or more commands in non-interactive mode
- `<?>` : see keyboard shortcuts (the most relevant are visible in the status line)

## Categories
text, network

## Links
- More information: https://newsboat.org/releases/2.40/docs/newsboat.html#_first_steps
- tldr page: https://tldr.inbrowser.app/pages/common/newsboat
