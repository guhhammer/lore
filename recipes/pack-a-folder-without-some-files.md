# pack a folder without some files

Make a tar archive of a folder, leaving out folders like node_modules and files like logs.

## Steps
- `tar -czf project.tar.gz --exclude='node_modules' --exclude='*.log' project/` : pack project without node_modules and the logs
- `tar -czf project.tar.gz --exclude-vcs project/` : without .git and the like (GNU tar)
- `tar -tzf project.tar.gz | less` : check what went in
- `git archive -o project.tar.gz HEAD` : in a git repository: only the files git tracks

## Notes
- Put the --exclude options before the folder: some tar versions ignore them after it.

## See also
- tar
- git

## Categories
archives files
