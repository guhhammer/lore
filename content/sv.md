# sv

Control a running runsv service.

## Usage
- `sudo sv up path/to/service` : start a service
- `sudo sv down path/to/service` : stop a service
- `sudo sv status path/to/service` : get service status
- `sudo sv reload path/to/service` : reload a service
- `sudo sv once path/to/service` : start a service, but only if it's not running and don't restart it if it stops

## Categories
system, files, users

## Links
- More information: https://manned.org/sv
- tldr page: https://tldr.inbrowser.app/pages/common/sv
