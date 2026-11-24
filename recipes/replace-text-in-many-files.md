# replace text in many files

Change a word or a phrase in every file under a folder that contains it.

## Steps
- `grep -rl 'old' .` : first, see which files contain it
- `grep -rl 'old' . | xargs sed -i 's/old/new/g'` : change it in all of them
- `find . -name '*.txt' -exec sed -i 's/old/new/g' {} +` : only in the .txt files
- `git diff` : in a git repository, check what changed

## Notes
- Run it in a git repository or on a copy: a wrong pattern changes many files at once.
- macOS: `sed -i ''` in place of `sed -i`.

## See also
- grep
- sed
- find
- xargs

## Categories
text files
