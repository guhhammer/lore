# xvfb-run

Run a command in a virtual X server environment.

## Usage
- `xvfb-run command` : run the specified command in a virtual X server
- `xvfb-run -a command` : try to get a free server number, if the default (99) is not available
- `xvfb-run -s "-screen 0 1024x768x24" command` : pass arguments to the Xvfb server

## Links
- More information: https://manned.org/xvfb-run
- tldr page: https://tldr.inbrowser.app/pages/linux/xvfb-run
