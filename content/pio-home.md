# pio home

Launch the PlatformIO Home web server.

## Usage
- `pio home` : open PlatformIO Home in the default web browser
- `pio home --port port` : use a specific HTTP port (defaults to 8008)
- `pio home --host ip_address` : bind to a specific IP address (defaults to 127.0.0.1)
- `pio home --no-open` : do not automatically open PlatformIO Home in the default web browser
- `pio home --shutdown-timeout time` : automatically shutdown the server on timeout (in seconds) when no clients are connected
- `pio home --session-id session_id` : specify a unique session identifier to keep PlatformIO Home isolated from other instances and protected from 3rd party access

## Categories
network

## Links
- More information: https://docs.platformio.org/en/latest/core/userguide/cmd_home.html
- tldr page: https://tldr.inbrowser.app/pages/common/pio-home
