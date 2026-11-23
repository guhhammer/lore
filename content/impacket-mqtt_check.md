# impacket-mqtt_check

This command is an alias of `mqtt_check.py`.

## Usage
- `mqtt_check.py domain/username:password@target_name` : check MQTT login credentials for a target (MQTT broker's hostname)
- `mqtt_check.py -client-id client_id domain/username:password@target_name` : specify a custom client ID for authentication
- `mqtt_check.py -ssl domain/username:password@target_name` : enable SSL for the connection
- `mqtt_check.py -port port domain/username:password@target_name` : connect to a specific port (default is 1883)
- `mqtt_check.py -debug domain/username:password@target_name` : enable debug output
- `mqtt_check.py --help` : display help

## Categories
users, network, shell

## Links
- tldr page: https://tldr.inbrowser.app/pages/common/impacket-mqtt_check
