# find recently changed files

The newest files in a folder, or those changed in the last minutes or days.

## Steps
- `ls -lt | head` : the newest here, first
- `ls -t | head -1` : only the newest one's name
- `find . -type f -mmin -60` : files changed in the last hour, under here
- `find . -type f -mtime -2` : changed in the last two days
- `find . -type f -printf '%T+ %p\n' | sort | tail -20` : the 20 newest under here, with dates (GNU)

## See also
- ls
- find

## Categories
files
