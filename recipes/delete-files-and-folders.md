# delete files and folders

Remove files and folders, ask before each, or remove huge folders quickly.

## Steps
- `rm file.txt` : delete a file
- `rm -ri folder` : delete a folder, asking for each file
- `rm -rf folder` : delete a folder and everything in it, without asking
- `find . -name '*.tmp' -delete` : delete every .tmp file under here
- `gio trash file.txt` : move it to the desktop's trash instead

## Notes
- There is no undo: check the path, mind spaces in names, and never run `rm -rf` with a variable that could be empty.
- "Argument list too long": see the recipe on that, or use find -delete.

## See also
- rm
- find
- gio

## Categories
files
