# rmpc

A terminal music player client for the Music Player Daemon, with album art support.

## Usage
- `rmpc` : start the TUI client
- `rmpc -a host:port` : connect to a specific MPD instance
- `rmpc config` : print the default configuration to use as a starting point
- `rmpc update` : scan the music directory for changes
- `rmpc albumart -o path/to/cover.jpg` : save the currently playing song's album art to a file
- `rmpc next|prev|stop|togglepause|toggleconsume|togglerepeat` : control playback (`toggleconsume` removes songs from the queue after they are played)
- `rmpc song` : view the currently playing song

## See also
- mpd
- mpc
- ncmpcpp

## Categories
media, network, files

## Links
- More information: https://rmpc.mierak.dev/reference/cli-command-mode/
- tldr page: https://tldr.inbrowser.app/pages/common/rmpc
