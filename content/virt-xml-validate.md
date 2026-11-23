# virt-xml-validate

Validate `libvirt` XML files against a schema. If a schema is not specified, the schema is determined by the root element in the XML file.

## Usage
- `virt-xml-validate path/to/file.xml schema` : validate an XML file against a specific schema
- `virt-xml-validate path/to/domain.xml domain` : validate the domain XML against the domain schema

## Categories
text, files

## Links
- More information: https://libvirt.org/manpages/virt-xml-validate.html
- tldr page: https://tldr.inbrowser.app/pages/linux/virt-xml-validate
