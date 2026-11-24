# xml edit

Edit an XML document.

## Usage
- `xml ed -d "XPATH1" path/to/input.xml|uri` : delete elements matching an XPATH from an XML document
- `xml ed -m "XPATH1" "XPATH2" path/to/input.xml|uri` : move an element node of an XML document from XPATH1 to XPATH2
- `xml ed -r "//*/@id" -v "ID" path/to/input.xml|uri` : rename all attributes named "id" to "ID"
- `xml ed -r "/xml/table/rec" -v "record" path/to/input.xml|uri` : rename sub-elements of the element "table" that are named "rec" to "record"
- `xml ed -u "xml/table/rec[@id=3]/@id" -v 5 path/to/input.xml|uri` : update the XML table record with "id=3" to the value "id=5"
- `xml ed -h` : display help

## Categories
text, files, documents

## Links
- More information: https://xmlstar.sourceforge.net/doc/UG/xmlstarlet-ug.html#idm47077139594320
- tldr page: https://tldr.inbrowser.app/pages/common/xml-edit
- Wikipedia: https://en.wikipedia.org/wiki/XML
