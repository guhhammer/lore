# varlinkctl

Introspect with and invoke Varlink services.

## Usage
- `varlinkctl [OPTIONS...] info ADDRESS` : the general form, as the manual gives it
- `varlinkctl [OPTIONS...] list-interfaces ADDRESS` : another form of the same command
- `varlinkctl [OPTIONS...] list-methods ADDRESS [INTERFACE...]` : another form of the same command
- `varlinkctl [OPTIONS...] introspect ADDRESS [INTERFACE...]` : another form of the same command
- `varlinkctl [OPTIONS...] call ADDRESS METHOD [ARGUMENTS]` : another form of the same command
- `varlinkctl --more` : when used with call: expect multiple method replies
- `varlinkctl -E` : a shortcut for --more --timeout=infinity

## Categories
system, network

## Links
- Documentation: https://systemd.io/
