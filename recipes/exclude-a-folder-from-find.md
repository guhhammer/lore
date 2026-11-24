# exclude a folder from find

Search with find without going into folders like .git or node_modules.

## Steps
- `find . -path ./node_modules -prune -o -name '*.js' -print` : every .js file, without going into node_modules
- `find . -not -path '*/.git/*' -name '*.md'` : every .md file outside .git folders (still walks them)
- `find . \( -name .git -o -name node_modules \) -prune -o -type f -print` : every file, skipping both folders
- `find . -name '*.log' 2>/dev/null` : hide the permission denied messages

## Notes
- `-prune` does not go into the folder at all, so it is fast on big trees; `-not -path` still walks it.

## See also
- find
- fd

## Categories
files search
