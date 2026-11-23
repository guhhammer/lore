# minidlna

ReadyMedia (formerly MiniDLNA) is a lightweight media server compatible with DLNA/UPnP-AV clients. Used to stream media to smart TVs, consoles, and other DLNA-compatible devices. Configuration is typically done via the `minidlna.conf` file.

## Usage
- `minidlna` : start the MiniDLNA daemon using the default configuration at `/etc/minidlna.conf`
- `minidlna -f path/to/minidlna.conf` : start MiniDLNA with a specific configuration file
- `minidlna -R` : force a database rescan on startup
- `minidlna -d` : run MiniDLNA in the foreground (useful for debugging)
- `minidlna -d -v` : enable verbose debug output
- `minidlna -m path/to/media` : specify a custom media directory (overrides config)
- `minidlna -P path/to/pidfile` : specify a custom database directory
- `minidlna -l path/to/logfil.log` : specify a custom log file path

## Categories
files, hardware, media

## Links
- More information: https://manned.org/minidlna
- tldr page: https://tldr.inbrowser.app/pages/common/minidlna
