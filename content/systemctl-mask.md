# systemctl mask

Link units to `/dev/null` so that they cannot be started.

## Usage
- `systemctl mask service_name` : mask a service
- `systemctl mask service_name --now` : ensure that the service is shut down while masking
- `systemctl mask service_name --user` : mask a user service

## See also
- systemctl revert

## Categories
files, system

## Links
- tldr page: https://tldr.inbrowser.app/pages/linux/systemctl-mask
