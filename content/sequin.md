# sequin

Make ANSI escape sequences human-readable. Useful for debugging, learning, or inspecting terminal output.

## Usage
- `printf "\x1b[38;5;4mCiao, \x1b[1;7mBaby.\x1b[0m\n" | sequin` : describe ANSI escape sequences from a string
- `ls -l --color=always | sequin` : inspect colorized output from another command (e.g., `ls`)
- `cat path/to/file.golden | sequin` : examine a file containing ANSI sequences (e.g., a TUI golden file)
- `sequin -- ls -l go.*` : execute a command directly within a fake TTY to inspect its output
- `git -c status.color=always status -sb | sequin -r` : highlight raw ANSI sequences inline for easier reading

## Categories
files, shell

## Links
- More information: https://github.com/charmbracelet/sequin
- tldr page: https://tldr.inbrowser.app/pages/common/sequin
