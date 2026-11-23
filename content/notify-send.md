# notify-send

Use the current desktop environment's notification system to create a notification.

## Usage
- `notify-send "Test" "This is a test"` : show a notification with the title "Test" and the content "This is a test"
- `notify-send -i icon.png "Test" "This is a test"` : show a notification with a custom icon
- `notify-send -t 5000 "Test" "This is a test"` : show a notification for 5 seconds
- `notify-send -u low|normal|critical "Test" "This is a test"` : show a notification with the specified urgency level (default: normal)
- `notify-send "Test" -i google-chrome -a "Google Chrome"` : show a notification with an app's icon and name

## Categories
desktop, development

## Links
- More information: https://manned.org/notify-send
- tldr page: https://tldr.inbrowser.app/pages/common/notify-send
