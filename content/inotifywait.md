# inotifywait

Wait for changes to files.

## Usage
- `inotifywait path/to/file` : watch a specific file for events, exiting after the first one
- `inotifywait -m path/to/file` : continuously watch a specific file for events without exiting
- `inotifywait -m -r path/to/directory` : watch a directory recursively for events
- `inotifywait -m -r --exclude "regex" path/to/directory` : watch a directory for changes, excluding files, whose names match a `regex`
- `inotifywait -m -t 30 path/to/file` : watch a file for changes, exiting when no event occurs for 30 seconds
- `inotifywait -e modify path/to/file` : only watch a file for file modification events
- `inotifywait -q path/to/file` : watch a file printing only events, and no status messages
- `inotifywait -e access path/to/file && command` : run a command when a file is accessed

## See also
- fatrace

## Categories
files

## Links
- More information: https://manned.org/inotifywait
- tldr page: https://tldr.inbrowser.app/pages/linux/inotifywait
