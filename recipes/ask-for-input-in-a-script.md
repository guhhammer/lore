# ask for input in a script

Ask yes or no, read a password without showing it, or wait for an answer with a time limit.

## Steps
- `read -rp 'Continue? [y/N] ' answer` : ask, and keep the answer
- `[[ $answer == [yY]* ]] || exit 1` : stop unless it starts with y
- `read -rsp 'Password: ' password; echo` : read a password without showing it
- `read -rt 10 -p 'Name: ' name || name=guest` : wait 10 seconds at most

## See also
- read

## Categories
shell development
