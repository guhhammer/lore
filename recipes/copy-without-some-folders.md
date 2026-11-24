# copy without some folders

Copy a folder but leave out some of what is inside, like node_modules or .git.

## Steps
- `rsync -av --exclude='node_modules' src/ dest/` : copy src into dest without node_modules
- `rsync -av --exclude={'.git','*.log'} src/ dest/` : without several things
- `rsync -avn --exclude='node_modules' src/ dest/` : only show what would be copied (-n)

## See also
- rsync
- cp

## Categories
files
