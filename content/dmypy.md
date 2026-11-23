# dmypy

Type check Python code, running `mypy` as a daemon for better speed.

## Usage
- `dmypy run -- path/to/file.py` : type check a file, and start the daemon if it is not running
- `dmypy start` : start the daemon
- `dmypy check -- path/to/file.py` : type check a file (requires the daemon to be running)
- `dmypy stop` : stop the daemon

## See also
- mypy

## Categories
system, files

## Links
- More information: https://mypy.readthedocs.io/en/stable/mypy_daemon.html
- tldr page: https://tldr.inbrowser.app/pages/common/dmypy
