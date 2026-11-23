# systemctl set-property

Set the specified unit properties at runtime.

## Usage
- `systemctl set-property unit property=value` : set a property for a running service
- `systemctl set-property unit property_1=value_1 property_2=value_2 ...` : set multiple properties at once
- `systemctl set-property unit property=value --runtime` : set a property only for the current runtime session (not persistent)
- `systemctl set-property unit property=` : reset a property to its default value
- `systemctl set-property unit property_1= property_2= ...` : reset multiple properties to its default values

## See also
- systemctl revert

## Links
- tldr page: https://tldr.inbrowser.app/pages/linux/systemctl-set-property
