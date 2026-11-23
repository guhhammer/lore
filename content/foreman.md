# foreman

Manage Procfile-based applications.

## Usage
- `foreman start` : start an application with the Procfile in the current directory
- `foreman start -f Procfile` : start an application with a specified Procfile
- `foreman start process` : start a specific application
- `foreman check` : validate Procfile format
- `foreman run command` : run one-off commands with the process's environment
- `foreman start -m all=1,worker=0` : start all processes except the one named "worker"

## Categories
processes

## Links
- More information: https://manned.org/foreman
- tldr page: https://tldr.inbrowser.app/pages/linux/foreman
