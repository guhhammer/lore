# x11vnc

A VNC server that will enable VNC on an existing display server. By default, the server will automatically terminate once all clients disconnect from it.

## Usage
- `x11vnc -shared` : launch a VNC server that allows multiple clients to connect
- `x11vnc -forever -viewonly` : launch a VNC server in view-only mode, and which won't terminate once the last client disconnects
- `x11vnc -display :display.screen` : launch a VNC server on a specific display and screen (both starting at index zero)
- `x11vnc -display :2` : launch a VNC server on the third display's default screen
- `x11vnc -display :0.1` : launch a VNC server on the first display's second screen

## Categories
desktop

## Links
- More information: https://manned.org/x11vnc
- tldr page: https://tldr.inbrowser.app/pages/linux/x11vnc
