# rm

Deletes files, and with `-r` folders. There is no trash and no undo: what
`rm` removes is gone.

## Usage
- `rm file.txt` : delete a file
- `rm -i *.log` : ask before each one
- `rm -r folder` : delete a folder and everything in it
- `rmdir folder` : delete a folder only if it is empty (safer)

## Notes
Read an `rm -rf` twice before you press Enter: a stray space, as in
`rm -rf / tmp`, deletes far more than meant. Run `ls` with the same
arguments first to see what would go.

## See also
- rmdir
- trash

## Categories
danger, files

## Links
- GNU manual: https://www.gnu.org/software/coreutils/rm
- Wikipedia: https://en.wikipedia.org/wiki/Rm_(Unix)
