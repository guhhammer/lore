# pueue add

Enqueue a task for execution.

## Usage
- `pueue add command` : add any command to the default queue
- `pueue add -- command --arg -f` : pass a list of flags or arguments to a command when enqueuing
- `pueue add -s -- rsync --archive --compress /local/directory /remote/directory` : add a command but do not start it if it's the first in a queue
- `pueue add -i -g "cpu_intensive" -- ffmpeg -i input.mp4 frame_%d.png` : add a command to a group and start it immediately, see `pueue group` to manage groups
- `pueue add -a 9 12 -g "torrents" -- transmission-cli torrent_file.torrent` : add a command and start it after commands 9 and 12 finish successfully
- `pueue add -l "compressing large file" -d "wednesday 10:30pm" -- "7z a compressed_file.7z large_file.xml"` : add a command with a label after some delay has passed, see `pueue enqueue` for valid datetime formats

## Categories
archives, files, users

## Links
- More information: https://github.com/Nukesor/pueue#how-to-use-it
- tldr page: https://tldr.inbrowser.app/pages/common/pueue-add
