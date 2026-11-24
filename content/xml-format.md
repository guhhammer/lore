# xml format

Format an XML document.

## Usage
- `xml fo -t path/to/input.xml|uri >path/to/output.xml` : format an XML document, indenting with tabs
- `xml fo -H -s 4 path/to/input.html|uri >path/to/output.html` : format an HTML document, indenting with 4 spaces
- `xml fo -R -n path/to/malformed.xml|uri >path/to/recovered.xml` : recover parsable parts of a malformed XML document, without indenting
- `cat path/to/input.xml | xml fo -D >path/to/output.xml` : format an XML document from `stdin`, removing the `DOCTYPE` declaration
- `xml fo -o path/to/input.xml|uri >path/to/output.xml` : format an XML document, omitting the XML declaration
- `xml fo --help` : display help

## Categories
text, files, documents

## Links
- More information: https://xmlstar.sourceforge.net/doc/UG/xmlstarlet-ug.html#idm47077139569312
- tldr page: https://tldr.inbrowser.app/pages/common/xml-format
- Wikipedia: https://en.wikipedia.org/wiki/XML
