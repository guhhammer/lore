# xml transform

Transform XML documents using XSLT.

## Usage
- `xml tr path/to/stylesheet.xsl -p "Count='count(/xml/table/rec)'" -s Text="Count=" path/to/input.xml|uri` : transform an XML document using an XSL stylesheet, passing one XPATH parameter and one literal string parameter
- `xml tr --help` : display help

## Categories
text

## Links
- More information: https://xmlstar.sourceforge.net/doc/UG/xmlstarlet-ug.html#idm47077139602800
- tldr page: https://tldr.inbrowser.app/pages/common/xml-transform
