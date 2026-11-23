# xml elements

Extract elements and display the structure of an XML document.

## Usage
- `xml el path/to/input.xml|uri >path/to/elements.xpath` : extract elements from an XML document (producing XPATH expressions)
- `xml el -a path/to/input.xml|uri >path/to/elements.xpath` : extract elements and their attributes from an XML document
- `xml el -v path/to/input.xml|uri >path/to/elements.xpath` : extract elements and their attributes and values from an XML document
- `xml el -u path/to/input.xml|uri` : print sorted unique elements from an XML document to see its structure
- `xml el -d3 path/to/input.xml|uri` : print sorted unique elements from an XML document up to a depth of 3
- `xml el --help` : display help

## Categories
text, documents, files

## Links
- More information: https://xmlstar.sourceforge.net/doc/UG/xmlstarlet-ug.html#idm47077139665568
- tldr page: https://tldr.inbrowser.app/pages/common/xml-elements
