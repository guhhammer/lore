# which shell is running

Find out which shell runs now, which one is your default, and change it.

## Steps
- `ps -p $$` : the shell running this terminal
- `echo "$SHELL"` : your login shell, the default
- `cat /etc/shells` : the shells you can choose
- `chsh -s /bin/zsh` : make zsh your default, from the next login

## See also
- ps
- chsh

## Categories
shell
