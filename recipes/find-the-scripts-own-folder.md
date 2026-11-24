# find the script's own folder

Inside a script, find the folder the script is in, to reach files next to it.

## Steps
- `dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"` : the script's folder, as a full path
- `cd "$dir"` : work from there
- `echo "$(basename "$0")"` : the script's own name
- `realpath "$0"` : its full path, links resolved

## See also
- dirname
- realpath

## Categories
shell development
