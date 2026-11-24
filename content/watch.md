# watch

Execute a program periodically and monitor the output in full-screen mode.

## Usage
- `watch command` : repeatedly run a command and show the result
- `watch -n 60 command` : re-run a command every 60 seconds
- `watch -d df` : monitor disk space, highlighting differences as they appear
- `watch "command1 | command2 | command3"` : repeatedly run a pipeline and show the result
- `watch -g lsblk` : exit `watch` if the visible output changes
- `watch -c ls --color=always` : interpret terminal control characters

## Categories
monitoring, shell

## Links
- More information: https://manned.org/watch
- tldr page: https://tldr.inbrowser.app/pages/common/watch
- Wikipedia: https://en.wikipedia.org/wiki/Watch_(command)
