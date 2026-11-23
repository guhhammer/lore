# pdfsig

Portable Document Format (PDF) digital signatures tool.

## Usage
- `pdfsig [options] [PDF-file] [Output-file]` : the general form, as the manual gives it
- `pdfsig -nss-pwd password` : specify the password needed to access the NSS database (if any)
- `pdfsig -nocert` : do not validate the certificate
- `pdfsig -dump` : dump all signatures into current directory in their native format
- `pdfsig -add-signature` : add a new signature to the document

## Categories
documents, media, files
