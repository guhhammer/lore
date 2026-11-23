# cpdf

Manipulate PDF files.

## Usage
- `cpdf path/to/source_document.pdf 1-3,6 -o path/to/destination_document.pdf` : select pages 1, 2, 3, and 6 from a source document and write those to a destination document
- `cpdf -merge path/to/source_document_one.pdf path/to/source_document_two.pdf -o path/to/destination_document.pdf` : merge two documents into a new one
- `cpdf -list-bookmarks path/to/document.pdf` : show the bookmarks of a document
- `cpdf -split path/to/document.pdf -o path/to/chunk%%%.pdf -chunk 10` : split a document into ten-page chunks, writing them to `chunk001.pdf`, `chunk002.pdf`, etc
- `cpdf -encrypt 128bit fred joe path/to/source_document.pdf -o path/to/encrypted_document.pdf` : encrypt a document using 128bit encryption, providing `fred` as owner password and `joe` as user password
- `cpdf -decrypt path/to/encrypted_document.pdf owner=fred -o path/to/decrypted_document.pdf` : decrypt a document using the owner password `fred`
- `cpdf -list-annotations path/to/document.pdf` : show the annotations of a document
- `cpdf -set-metadata path/to/metadata.xml path/to/source_document.pdf -o path/to/destination_document.pdf` : create a new document from an existing one with additional metadata

## Categories
documents, media, files

## Links
- More information: https://www.coherentpdf.com/cpdfmanual/cpdfmanual.html
- tldr page: https://tldr.inbrowser.app/pages/common/cpdf
