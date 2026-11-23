# grafana server

Start the Grafana web server. Run this from the Grafana installation directory, or specify `--homepath`.

## Usage
- `grafana server` : start the Grafana server
- `grafana server --config path/to/grafana.ini` : start with a specific config file
- `grafana server --homepath path/to/grafana` : start with a specific homepath (defaults to the working directory)
- `grafana server --pidfile path/to/file.pid` : write the server process ID to a file
- `grafana server --configOverrides cfg:default.paths.log=path/to/logfile` : override configuration options
- `grafana server --vv` : print the current version along with all dependency versions

## See also
- grafana
- grafana cli

## Categories
files

## Links
- More information: https://grafana.com/docs/grafana/latest/setup-grafana/start-restart-grafana/
- tldr page: https://tldr.inbrowser.app/pages/common/grafana-server
