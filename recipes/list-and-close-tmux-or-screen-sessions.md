# list and close tmux or screen sessions

See the terminal sessions left running, join one, or end them.

## Steps
- `tmux ls` : the tmux sessions
- `tmux attach -t NAME` : join one
- `tmux kill-session -t NAME` : end one
- `screen -ls` : the screen sessions
- `screen -r NAME` : join one
- `screen -X -S NAME quit` : end one, from outside

## See also
- tmux
- screen

## Categories
processes shell
