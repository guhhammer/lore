# cp

Copies files, and with `-r` whole folders. The copy replaces a file of the
same name at the destination without asking, unless you add `-i`.

## Usage
- `cp notes.txt backup.txt` : copy a file under a new name
- `cp notes.txt ~/Documents/` : copy it into a folder
- `cp -r photos photos-copy` : copy a folder and everything in it
- `cp -i a.txt b.txt` : ask before replacing a file
- `cp -a src dest` : keep permissions, owners and dates too

## Categories
files

## Links
- GNU manual: https://www.gnu.org/software/coreutils/cp
- Wikipedia: https://en.wikipedia.org/wiki/Cp_(Unix)
