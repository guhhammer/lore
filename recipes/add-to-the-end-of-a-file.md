# add to the end of a file

Add lines to a file without replacing it, also to a file only root can write.

## Steps
- `echo 'new line' >> file.txt` : add a line at the end
- `command >> log.txt 2>&1` : add a command's output and errors
- `command | tee -a log.txt` : see it and add it to the file
- `echo 'new line' | sudo tee -a /etc/some.conf` : add to a file only root can write

## Notes
- One `>` replaces the file; two, `>>`, add to it.
- `sudo echo x >> /etc/file` fails: the redirection is done by your shell, not by sudo.

## See also
- tee
- echo

## Categories
files shell
