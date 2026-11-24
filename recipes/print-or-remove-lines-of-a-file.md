# print or remove lines of a file

Show one line by its number, a range, skip the first ones, or take off the last.

## Steps
- `sed -n '5p' file.txt` : line 5
- `sed -n '5,10p' file.txt` : lines 5 to 10
- `tail -n +2 file.txt` : everything from line 2: skip a header
- `head -n 20 file.txt` : the first 20 lines
- `tail -n 20 file.txt` : the last 20 lines
- `sed -i '$d' file.txt` : remove the last line
- `head -n -5 file.txt` : everything but the last 5 lines (GNU)

## See also
- sed
- head
- tail
- awk

## Categories
text
