# chmod

Changes who may read, write or run a file: its owner (u), its group (g)
and everyone else (o).

## Usage
- `chmod +x script.sh` : let it be run as a program
- `chmod 644 file` : owner reads and writes, everyone else reads
- `chmod 755 folder` : owner does everything, others read and enter
- `chmod -R u+rw folder` : give the owner read and write on everything inside
- `chmod go-r secret.txt` : stop group and others reading it

## Categories
files, users

## Links
- GNU manual: https://www.gnu.org/software/coreutils/chmod
