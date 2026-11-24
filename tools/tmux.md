# tmux

tmux keeps terminal sessions alive in the background, so a closed window or a dropped ssh connection loses nothing. It also splits one terminal into panes and windows. Key presses start with the prefix, Ctrl+b.

## Tasks
### Sessions
- `tmux new -s NAME` : start a named session
- `tmux ls` : list sessions
- `tmux attach -t NAME` : go back to a session
- `Ctrl+b d` : detach, leaving the session running
- `tmux kill-session -t NAME` : end a session
- `Ctrl+b $` : rename the session
- `Ctrl+b s` : pick a session from a list
### Windows
- `Ctrl+b c` : new window
- `Ctrl+b n` : next window
- `Ctrl+b p` : previous window
- `Ctrl+b 0` : jump to window 0 (any digit)
- `Ctrl+b ,` : rename the window
- `Ctrl+b &` : close the window
### Panes
- `Ctrl+b %` : split side by side
- `Ctrl+b "` : split top and bottom
- `Ctrl+b o` : go to the next pane
- `Ctrl+b Arrow` : move to the pane in that direction
- `Ctrl+b z` : zoom a pane to full size, again to restore
- `Ctrl+b x` : close the pane
- `Ctrl+b Ctrl+Arrow` : resize the pane
### Scroll and copy
- `Ctrl+b [` : scroll mode; arrows or PgUp move, q leaves
- `Ctrl+b ]` : paste tmux's last copy
- `Ctrl+b :` : the command prompt
- `Ctrl+b ?` : list every key
### Settings
- `tmux source-file ~/.tmux.conf` : reload the config
- `tmux set -g mouse on` : use the mouse for panes and scrolling
- `tmux new -A -s NAME` : attach to NAME or make it when missing

## Notes
- Press the prefix, let go, then press the next key: `Ctrl+b` then `d`.
- Shift the prefix to something nearer with `set -g prefix C-a` in `~/.tmux.conf`.
- Over ssh, run tmux on the server: a dropped connection keeps your work running.
- Inside scroll mode, copy keys depend on `mode-keys` (emacs or vi).

## See also
- screen
- ssh
- vim
- zellij

## Links
- Wiki: https://github.com/tmux/tmux/wiki
- Manual: https://man7.org/linux/man-pages/man1/tmux.1.html

## Categories
shell
