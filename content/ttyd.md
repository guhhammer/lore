# ttyd

Share a terminal or any command over the web.

## Usage
- `ttyd bash` : start a read only web server sharing Bash shell on the default port (7681)
- `ttyd -p 8080 bash` : start Bash on a specific port
- `ttyd -W command` : allow clients to write to the terminal
- `ttyd -t key=value command` : set client options
- `ttyd -h` : display help
- `ttyd -v` : display version

## Categories
shell, network

## Links
- More information: https://github.com/tsl0922/ttyd#command-line-options
- tldr page: https://tldr.inbrowser.app/pages/common/ttyd
