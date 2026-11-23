# attr

Manipulate extended attributes on filesystem objects.

## Usage
- `attr -l path/to/file` : list the names and sizes of all extended attributes of a file
- `attr -g attribute_name path/to/file` : get the value of an extended attribute
- `attr -s attribute_name -V "attribute_value" path/to/file` : set an extended attribute value
- `echo "attribute_value" | attr -s attribute_name path/to/file` : set an extended attribute using a value from `stdin`
- `attr -r attribute_name path/to/file` : remove an extended attribute

## Categories
files, disks

## Links
- More information: https://manned.org/attr
- tldr page: https://tldr.inbrowser.app/pages/common/attr
