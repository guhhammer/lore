# jwebserver

Starts a minimal web server that serves the files of a folder, for testing.

## Usage
- `jwebserver` : serve the current folder on port 8000
- `jwebserver -p 9000` : use another port
- `jwebserver -d /path/to/folder` : serve another folder
- `jwebserver -b 0.0.0.0` : accept connections from other machines

## Notes
By default it listens only on this machine. Binding to 0.0.0.0 shares the folder with the network.

## Categories
files, network
