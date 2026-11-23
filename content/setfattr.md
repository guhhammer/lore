# setfattr

Set extended file attributes.

## Usage
- `setfattr -n user.attribute_name path/to/file` : set name of attribute for file
- `setfattr -n user.attribute_name -v "value" path/to/file` : set a user-defined value of an extended attribute on a file
- `setfattr -x user.attribute_name path/to/file` : remove a specific attribute of a file

## See also
- getfattr

## Categories
files, users

## Links
- More information: https://manned.org/setfattr
- tldr page: https://tldr.inbrowser.app/pages/linux/setfattr
