# run-mailcap

Execute programs via entries in the mailcap file.

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
files, documents

## Links
- More information: https://manned.org/run-mailcap
- tldr page: https://tldr.inbrowser.app/pages/linux/run-mailcap
