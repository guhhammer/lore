# git instaweb

Helper to launch a GitWeb server.

## Usage
- `git instaweb --start` : launch a GitWeb server for the current Git repository
- `git instaweb --start -l` : listen only on localhost
- `git instaweb --start -p 1234` : listen on a specific port
- `git instaweb --start -d lighttpd|apache2|mongoose|plackup|webrick` : use a specified HTTP daemon
- `git instaweb --start -b` : also auto-launch a web browser
- `git instaweb --stop` : stop the currently running GitWeb server
- `git instaweb --restart` : restart the currently running GitWeb server

## Categories
version-control, network

## Links
- More information: https://git-scm.com/docs/git-instaweb
- tldr page: https://tldr.inbrowser.app/pages/common/git-instaweb
- Wikipedia: https://en.wikipedia.org/wiki/Git
