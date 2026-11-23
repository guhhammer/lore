# rc-service

Locate and run OpenRC services with arguments.

## Usage
- `rc-service service_name status` : show a service's status
- `sudo rc-service service_name start` : start a service
- `sudo rc-service service_name stop` : stop a service
- `sudo rc-service service_name restart` : restart a service
- `sudo rc-service -Z service_name command_name` : simulate running a service's custom command
- `sudo rc-service service_name command_name` : actually run a service's custom command
- `sudo rc-service -r service_name` : resolve the location of a service definition on disk

## See also
- openrc

## Categories
system, users

## Links
- More information: https://manned.org/rc-service
- tldr page: https://tldr.inbrowser.app/pages/linux/rc-service
