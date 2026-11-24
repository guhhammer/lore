# keep a command running after logout

Start a long job so it survives closing the terminal or the ssh session, and come back to it.

## Steps
- `nohup long-command > out.log 2>&1 &` : start it in the background, immune to the hang-up
- `tmux new -s work` : or start it inside a tmux session; Ctrl+B then D leaves it running
- `tmux attach -t work` : come back to it later, even from another ssh login
- `screen -S work` : the same with screen; Ctrl+A then D leaves it
- `screen -r work` : come back to the screen session
- `disown -h %1` : already running? Ctrl+Z, then bg, then this, and it survives logout

## See also
- nohup
- tmux
- screen
- disown

## Categories
processes shell
