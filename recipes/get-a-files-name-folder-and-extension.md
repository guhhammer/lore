# get a file's name, folder and extension

Take a path apart in a script: the folder, the name, the name without its extension, the extension.

## Steps
- `file="${path##*/}"` : the name: /a/b/report.tar.gz → report.tar.gz
- `dir="${path%/*}"` : the folder: /a/b
- `ext="${file##*.}"` : the last extension: gz
- `base="${file%.*}"` : the name without it: report.tar
- `basename "$path" .gz` : the same with basename, ending removed
- `dirname "$path"` : the folder, with dirname

## Notes
- ## and %% take the longest match, # and % the shortest: ${file%%.*} gives report.

## See also
- basename
- dirname

## Categories
shell files development
