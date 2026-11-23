# ntfy

Send and receive HTTP POST notifications.

## Usage
- `ntfy pub security "Front door has been opened."` : send a message to the `security` topic
- `ntfy publish --title="Someone bought your item" --priority=high --tags=duck ebay "Someone just bought your item: Platypus Sculpture"` : send with a title, priority, and tags
- `ntfy pub --at=8:30am delayed_topic "Time for school, sleepyhead..."` : send at 8:30am
- `ntfy trigger my_webhook` : trigger a webhook
- `ntfy sub home_automation` : subscribe to a topic (`<Ctrl c>` to stop listening)
- `ntfy --help` : display help

## Categories
network

## Links
- More information: https://github.com/binwiederhier/ntfy
- tldr page: https://tldr.inbrowser.app/pages/common/ntfy
