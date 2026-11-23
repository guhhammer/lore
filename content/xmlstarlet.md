# xmlstarlet

An XML/XSLT toolkit. Note: You will likely need to know XPath: <https://developer.mozilla.org/en-US/docs/Web/XPath>.

## Usage
- `xmlstarlet format path/to/file.xml` : format an XML document and print to `stdout`
- `cat path/to/file.xml | xmlstarlet format` : format an XML document piped from `stdin`
- `xmlstarlet select --template --copy-of xpath path/to/file.xml` : print all nodes that match a given XPath
- `xmlstarlet edit --insert xpath --type attr --name attribute_name --value attribute_value path/to/file.xml` : insert an attribute to all matching nodes, and print to `stdout` (source file is unchanged)
- `xmlstarlet edit --inplace --update xpath --value new_value file.xml` : update the value of all matching nodes in place (source file is changed)
- `xmlstarlet edit --inplace --delete xpath file.xml` : delete all matching nodes in place (source file is changed)
- `xmlstarlet [un]escape string` : escape or unescape special XML characters in a given string
- `xmlstarlet ls path/to/directory` : list a given directory as XML (omit argument to list current directory)

## Categories
files, text, documents

## Links
- More information: https://xmlstar.sourceforge.net/docs.php
- tldr page: https://tldr.inbrowser.app/pages/common/xmlstarlet
