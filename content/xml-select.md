# xml select

Select from XML documents using XPATHs. Tip: use `xml elements` to display the XPATHs of an XML document.

## Usage
- `xml sel -t -m "XPATH1" -v "XPATH2" path/to/input.xml|uri` : select all elements matching "XPATH1" and print the value of their sub-element "XPATH2"
- `xml sel -T -t -m "XPATH1" -v "XPATH2" -n path/to/input.xml|uri` : match "XPATH1" and print the value of "XPATH2" as text with new-lines
- `xml sel -t -v "count(XPATH1)" path/to/input.xml|uri` : count the elements of "XPATH1"
- `xml sel -T -t -f -o " " -v "count(node())" -n path/to/input1.xml|uri path/to/input2.xml|uri` : count all nodes in one or more XML documents
- `xml sel --help` : display help

## Categories
text, documents, files

## Links
- More information: https://xmlstar.sourceforge.net/doc/UG/xmlstarlet-ug.html#idm47077139652416
- tldr page: https://tldr.inbrowser.app/pages/common/xml-select
- Wikipedia: https://en.wikipedia.org/wiki/XML
