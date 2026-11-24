# xephyr

A nested X server that runs as an X application.

## Usage
- `Xephyr -br -ac -noreset -screen 800x600 :2` : create a black window with display ID ":2"
- `DISPLAY=:2 command_name` : start an X application on the new screen

## Categories
desktop

## Links
- More information: https://manned.org/xserver-xephyr
- tldr page: https://tldr.inbrowser.app/pages/common/xephyr
- Wikipedia: https://en.wikipedia.org/wiki/Xephyr
