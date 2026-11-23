# see

This command is an alias of `run-mailcap --action=view`.

## Usage
- `run-mailcap --action=compose path/to/file` : compose any existing file or new on default `mailcap` edit tool
- `run-mailcap --action=edit path/to/file` : view any file on default `mailcap` explorer
- `run-mailcap --action=print path/to/file` : print a file using the default `mailcap` print tool
- `run-mailcap --action=view path/to/file` : view any file (usually image) on default `mailcap` explorer
- `run-mailcap --action=view|cat|compose|composetyped|edit|print path/to/file` : invoke individual actions/programs on run-mailcap
- `run-mailcap --action=action --debug path/to/file` : turn on extra information
- `run-mailcap --action=action --nopager path/to/file` : ignore any "copiousoutput" directive and forward output to `stdout`
- `run-mailcap --action=action --norun path/to/file` : display the found command without actually executing it

## Categories
files, documents, shell

## Links
- tldr page: https://tldr.inbrowser.app/pages/linux/see
