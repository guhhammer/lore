# pidwait

Look up, signal, or wait for processes based on name and other attributes.

## Usage
- `pgrep [options] pattern pkill [options] pattern` : the general form, as the manual gives it
- `pidwait [options] pattern` : another form of the same command
- `pidwait --signal signal` : defines the signal to send to each matched process
- `pidwait -e` : display name and PID of the process being killed
- `pidwait -f` : the pattern is normally only matched against the process name
- `pidwait -i` : match processes case-insensitively

## Categories
processes, text
