# count lines of code

How many lines of code a project has, in total and by language.

## Steps
- `cloc .` : lines of code, comments and blanks, by language (cloc)
- `tokei` : the same, faster (tokei)
- `git ls-files | xargs wc -l | tail -1` : the lines of every file git tracks
- `find . -name '*.py' -print0 | xargs -0 wc -l | tail -1` : the lines of every .py file

## See also
- cloc
- tokei
- wc
- git

## Categories
development text
