# compare two folders

Find which files differ between two folders, or are only in one of them.

## Steps
- `diff -rq dir1 dir2` : the files that differ, and those only in one
- `diff -r dir1 dir2` : the same, with the lines that differ
- `diff -u old.txt new.txt` : two files, the changed lines in context
- `rsync -rcn --out-format='%n' dir1/ dir2/` : what a copy from dir1 would change in dir2, by content

## Notes
- meld, if installed, shows the same side by side.

## See also
- diff
- rsync
- cmp

## Categories
files text
