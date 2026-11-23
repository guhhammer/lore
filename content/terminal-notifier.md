# terminal-notifier

Send macOS User Notifications.

## Usage
- `terminal-notifier -group tldr-info -title TLDR -message 'TLDR rocks'` : send a notification (only the message is required)
- `echo 'Piped Message Data!' | terminal-notifier -sound default` : display piped data with a sound
- `terminal-notifier -message 'Check your Apple stock!' -open 'http://finance.yahoo.com/q?s=AAPL'` : open a URL when the notification is clicked
- `terminal-notifier -message 'Imported 42 contacts.' -activate com.apple.AddressBook` : open an app when the notification is clicked

## Categories
shell, desktop, network, macos

## Links
- More information: https://github.com/julienXX/terminal-notifier
- tldr page: https://tldr.inbrowser.app/pages/osx/terminal-notifier
