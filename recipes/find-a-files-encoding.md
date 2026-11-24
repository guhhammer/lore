# find a file's encoding

See which character encoding a text file is in, and convert it to UTF-8.

## Steps
- `file -i file.txt` : the kind of file and its charset (macOS: file -I)
- `iconv -f ISO-8859-1 -t UTF-8 in.txt > out.txt` : convert it to UTF-8
- `iconv -l` : the encodings iconv knows
- `uchardet file.txt` : a better guess, if uchardet is installed

## Notes
- No tool can be sure of an encoding: it is a guess from the bytes.

## See also
- file
- iconv

## Categories
text
