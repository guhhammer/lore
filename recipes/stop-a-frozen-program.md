# stop a frozen program

Find a program that stopped answering and end it: politely first, by force
if it must.

## Steps
- `pgrep -l NAME` : the PIDs of the programs with that name
- `kill PID` : ask it to quit, so it can save and clean up
- `kill -9 PID` : force it to stop at once, if it did not quit
- `pkill NAME` : ask every program with that name to quit
- `xkill` : click a window to close its program (X11 desktops)

## Notes
- Put the program's name in place of NAME, and the number from pgrep in place of PID.
- A force-stopped program loses what it had not saved.
- If the whole desktop is frozen on Linux, Ctrl+Alt+F3 often opens a text console where you can log in and do the same.
- macOS: Option+Command+Esc, or the same commands.

## See also
- kill
- pkill
- pgrep
- killall

## Categories
processes
