# fix windows line endings

"bad interpreter: ^M" and odd characters: the file has Windows line endings (CRLF). Find them and change them to Unix ones (LF).

## Steps
- `file script.sh` : says "with CRLF line terminators" when it has them
- `cat -A script.sh | head` : shows them as ^M$ at the ends of the lines
- `dos2unix script.sh` : change them, if dos2unix is installed
- `sed -i 's/\r$//' script.sh` : the same with sed
- `unix2dos file.txt` : the other way, for Windows

## Notes
- git can do it for you: `git config --global core.autocrlf input`.

## See also
- dos2unix
- file
- sed

## Categories
text development
