# xfreerdp3

Free Remote Desktop Protocol implementation (version 3).

## Usage
- `xfreerdp3 /u:username /p:password /v:host` : connect to a FreeRDP server
- `xfreerdp3 /u:username /p:password /v:host /f` : connect to a server in fullscreen mode
- `xfreerdp3 /u:username /p:password /v:host /w:width_in_pixels /h:height_in_pixels` : connect to a server with custom display width and height
- `xfreerdp3 /u:username /p:password /v:host /dynamic-resolution` : connect to a server with dynamic resolution scaling
- `xfreerdp3 /u:username /p:password /v:host +clipboard` : connect to a server with clipboard redirection
- `xfreerdp3 /u:username /p:password /d:domain /v:host` : connect to a server with a specific domain
- `xfreerdp3 /u:username /p:password /v:host /cert:ignore` : connect to a server ignoring certificate checks

## Categories
network, users, desktop

## Links
- More information: https://manned.org/xfreerdp3
- tldr page: https://tldr.inbrowser.app/pages/linux/xfreerdp3
