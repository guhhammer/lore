# braa

Ultra-fast mass SNMP scanner allowing multiple hosts simultaneously.

## Usage
- `braa public@ip_address:.1.3.6.*` : walk the SNMP tree of host with public string querying all OIDs under `.1.3.6`
- `braa public@ip_range:.1.3.6.1.2.1.1.6.0` : query the whole subnet `ip_range` for `system.sysLocation.0`
- `braa private@ip_address:.1.3.6.1.2.1.1.6.0=s'workgroup'` : attempt to set the value of `system.sysLocation.0` to a specific workgroup

## Categories
network

## Links
- More information: https://manned.org/braa
- tldr page: https://tldr.inbrowser.app/pages/linux/braa
