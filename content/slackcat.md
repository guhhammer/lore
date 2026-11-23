# slackcat

Utility for passing files and command output to Slack.

## Usage
- `slackcat -c channel_name path/to/file` : post a file to Slack
- `slackcat -c channel_name -n filename path/to/file` : post a file to Slack with a custom filename
- `command | slackcat -c channel_name -n snippet_name` : pipe command output to Slack as a text snippet
- `command | slackcat -c channel_name -s` : stream command output to Slack continuously

## Categories
files

## Links
- More information: https://github.com/bcicen/slackcat#usage
- tldr page: https://tldr.inbrowser.app/pages/common/slackcat
