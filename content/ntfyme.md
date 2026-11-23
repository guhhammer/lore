# ntfyme

A notification tool to track and notify you about your long-running termination process. Send notifications with success/error messages with Gmail, Telegram, and more.

## Usage
- `ntfyme exec -c command` : directly run your command
- `echo command | ntfyme exec` : pipe your command and run
- `echo "command1; command2; command3" | ntfyme exec` : run multiple commands by enclosing them in quotes
- `ntfyme exec -t -c command` : track and terminate the process after prolonged suspension
- `ntfyme setup` : set up the tool configurations interactively
- `ntfyme enc` : encrypt your password
- `ntfyme log` : see the log history
- `ntfyme config` : open and edit the configuration file

## Categories
processes, desktop

## Links
- More information: https://github.com/AnirudhG07/ntfyme
- tldr page: https://tldr.inbrowser.app/pages/common/ntfyme
