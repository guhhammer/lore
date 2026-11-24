# rename many files

Change the ending of many files, replace spaces in names, or number them.

## Steps
- `for f in *.txt; do mv -- "$f" "${f%.txt}.md"; done` : every .txt to .md
- `rename 's/\.txt$/.md/' *.txt` : the same with rename (the perl one; package rename)
- `for f in *\ *; do mv -- "$f" "${f// /_}"; done` : spaces in names to underscores
- `rename -n 's/ /_/g' *` : show what rename would do, without doing it

## Notes
- There are two rename programs: the perl one takes s/old/new/, util-linux's takes old new files.

## See also
- mv
- rename

## Categories
files shell
