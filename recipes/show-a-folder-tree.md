# show a folder tree

Show folders and files as a tree, to see how a project is laid out.

## Steps
- `tree` : everything under here, as a tree
- `tree -L 2` : two levels down
- `tree -d` : folders only
- `tree -a -I 'node_modules|.git'` : hidden ones too, without those folders
- `find . -maxdepth 2 | sort` : without tree

## See also
- tree
- find

## Categories
files
