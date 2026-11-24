# parallel

Run commands on multiple CPU cores.

## Usage
- `parallel gzip ::: path/to/file1 path/to/file2 ...` : gzip several files at once, using all cores
- `ls *.txt | parallel -j 4 gzip` : read arguments from `stdin`, run 4 jobs at once
- `parallel convert {} {.}.png ::: *.jpg` : convert JPEG images to PNG using replacement strings
- `args | parallel -X command` : run like `xargs` in parallel, cramming as many arguments as possible onto one command
- `cat big_file.txt | parallel --pipe --block 1M command` : break `stdin` into ~1M blocks, feed each block to `stdin` of new command
- `parallel -S machine1,machine2 command ::: arg1 arg2` : run on multiple machines via SSH
- `parallel -j 4 --bar --eta curl -sO {} :::: path/to/links.txt` : download 4 files simultaneously from a text file containing links showing progress
- `parallel -t command ::: args` : print the jobs which `parallel` is running in `stderr`

## See also
- xargs

## Categories
files, network, archives

## Links
- More information: https://www.gnu.org/software/parallel/man.html
- tldr page: https://tldr.inbrowser.app/pages/common/parallel
- Wikipedia: https://en.wikipedia.org/wiki/GNU_parallel
