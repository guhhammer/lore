# xml canonic

Make XML documents canonical.

## Usage
- `xml c14n path/to/input.xml|uri >path/to/output.xml` : make an XML document canonical, preserving comments
- `xml c14n --without-comments path/to/input.xml|uri >path/to/output.xml` : make an XML document canonical, removing comments
- `xml c14n --exc-with-comments path/to/input.xml|uri path/to/c14n.xpath` : make XML exclusively canonical, using an XPATH from a file, preserving comments
- `xml c14n --help` : display help

## Categories
text, files

## Links
- More information: https://xmlstar.sourceforge.net/doc/UG/xmlstarlet-ug.html#idm47077139560880
- tldr page: https://tldr.inbrowser.app/pages/common/xml-canonic
- Wikipedia: https://en.wikipedia.org/wiki/XML
