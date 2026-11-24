# uuid

Generate and decode Universally Unique Identifiers (UUID).

## Usage
- `uuid` : generate a UUIDv1 (based on time and system's hardware address, if present)
- `uuid -v 4` : generate a UUIDv4 (based on random data)
- `uuid -v 4 -n number_of_uuids` : generate multiple UUIDv4 identifiers at once
- `uuid -v 4 -F BIN|STR|SIV` : generate a UUIDv4 and specify the output format
- `uuid -v 4 -o path/to/file` : generate a UUIDv4 and write the output to a file
- `uuid -v 5 ns:DNS|URL|OID|X500 object_name` : generate a UUIDv5 (based on the supplied object name) with a specified namespace prefix
- `uuid -d uuid` : decode a given UUID

## See also
- uuidgen

## Categories
network, files

## Links
- More information: https://manned.org/uuid
- tldr page: https://tldr.inbrowser.app/pages/linux/uuid
- Wikipedia: https://en.wikipedia.org/wiki/Universally_unique_identifier
