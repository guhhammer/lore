# search but skip some files

grep through a project without its build folders, minified files or logs.

## Steps
- `grep -rn --exclude-dir={.git,node_modules,dist} 'text' .` : skip those folders
- `grep -rn --include='*.js' --include='*.ts' 'text' .` : only those kinds of files
- `grep -rn --exclude='*.min.js' 'text' .` : skip files by name
- `rg 'text' -g '!*.min.js'` : ripgrep: skips .gitignore's files already; -g adds more

## See also
- grep
- rg

## Categories
text search
