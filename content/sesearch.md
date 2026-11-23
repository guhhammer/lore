# sesearch

Search SELinux policy rules. Part of the `setools` package.

## Usage
- `sesearch --allow` : search for all allow rules
- `sesearch --allow -t type_name` : search for rules related to a specific type
- `sesearch --allow -s source_type` : search for rules related to a specific source type
- `sesearch --allow -c class_name -p permission` : search for rules that allow a specific class and permission
- `sesearch --allow -t type_name -c class_name` : search for rules with a specific target type and class
- `sesearch --allow -t type_name -v` : display more detailed information about matched rules

## See also
- semodule

## Categories
packages

## Links
- More information: https://manned.org/sesearch
- tldr page: https://tldr.inbrowser.app/pages/linux/sesearch
