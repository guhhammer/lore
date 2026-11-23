# xml validate

Validate XML documents.

## Usage
- `xml val path/to/input1.xml|uri1 path/to/input2.xml|uri2 ...` : validate one or more XML documents for well-formedness only
- `xml val -d path/to/schema.dtd path/to/input1.xml|uri1 path/to/input2.xml|uri2 ...` : validate one or more XML documents against a Document Type Definition (DTD)
- `xml val -s path/to/schema.xsd path/to/input1.xml|uri1 path/to/input2.xml|uri2 ...` : validate one or more XML documents against an XML Schema Definition (XSD)
- `xml val -r path/to/schema.rng path/to/input1.xml|uri1 path/to/input2.xml|uri2 ...` : validate one or more XML documents against a Relax NG schema (RNG)
- `xml val --help` : display help

## Categories
text, files

## Links
- More information: https://xmlstar.sourceforge.net/doc/UG/xmlstarlet-ug.html#idm47077139576400
- tldr page: https://tldr.inbrowser.app/pages/common/xml-validate
