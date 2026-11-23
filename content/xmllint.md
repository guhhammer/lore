# xmllint

XML parser and linter that supports XPath, a syntax for navigating XML trees.

## Usage
- `xmllint --xpath "//node_name" source_file.xml` : return all nodes (tags) with a specific name
- `xmllint --xpath "string(//node_name)" source_file.xml` : return the contents of the first node with a specific name as a string
- `xmllint --html --xpath "string(//a[2]/@href)" webpage.xhtml` : return the href attribute of the second anchor element in an HTML file
- `xmllint --format source_file.xml` : return human-readable (indented) XML from file
- `xmllint --valid source_file.xml` : check that an XML file meets the requirements of its DOCTYPE declaration
- `xmllint --dtdvalid url source_file.xml` : validate XML against DTD schema hosted online

## Categories
text, files, network

## Links
- More information: https://manned.org/xmllint
- tldr page: https://tldr.inbrowser.app/pages/common/xmllint
