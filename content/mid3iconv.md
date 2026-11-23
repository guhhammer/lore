# mid3iconv

Convert ID3 tag encodings.

## Usage
- `mid3iconv [options] filename ...` : the general form, as the manual gives it
- `mid3iconv --debug` : print updated tags
- `mid3iconv --dry-run` : do not actually modify files
- `mid3iconv --encoding` : convert from this encoding
- `mid3iconv --force-v1` : use an ID3v1 tag even if an ID3v2 tag is present
