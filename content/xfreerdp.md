# xfreerdp

Free Remote Desktop Protocol implementation.

## Usage
- `xfreerdp /u:username /p:password /v:host` : connect to a FreeRDP server
- `xfreerdp /v:host /u:username /p:password /w:width_in_pixels /h:height_in_pixels` : connect to a FreeRDP server with a custom window width and height
- `xfreerdp /v:host /u:username /p:password /dynamic-resolution` : connect to a FreeRDP server with dynamic resolution
- `xfreerdp /v:host /u:username /p:password +clipboard` : connect to a FreeRDP server with clipboard redirection
- `xfreerdp /v:host /u:username /p:password /cert:ignore` : connect to a FreeRDP server ignoring any certificate checks
- `xfreerdp /v:host /u:username /p:password /drive:path/to/directory,share_name` : connect to a FreeRDP server with a shared directory
- `xfreerdp /u:username /p:password /v:host /sound:sys:alsa` : connect to a FreeRDP server and activate audio output redirection using `sys:alsa` device

## Categories
network, users, desktop

## Links
- tldr page: https://tldr.inbrowser.app/pages/linux/xfreerdp
