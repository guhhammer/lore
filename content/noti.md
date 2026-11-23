# noti

Monitor a process and trigger a banner notification.

## Usage
- `noti tar -cjf example.tar.bz2 example/` : display a notification when `tar` finishes compressing files
- `command_to_watch; noti` : display a notification even when you put it after the command to watch
- `noti -w process_id` : monitor a process by PID and trigger a notification when the PID disappears

## Categories
desktop, processes, monitoring

## Links
- More information: https://github.com/variadico/noti/blob/main/docs/noti.md
- tldr page: https://tldr.inbrowser.app/pages/common/noti
