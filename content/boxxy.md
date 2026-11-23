# boxxy

Redirect file locations for programs that don't respect the XDG standards.

## Usage
- `boxxy program` : run a program with file redirections defined in `~/.config/boxxy/boxxy.yaml`
- `boxxy scan` : scan your home directory for rule suggestions
- `boxxy -t program` : trace what files a program touches and save a report in `boxxy-report.txt` in the current directory
- `boxxy -r path/to/file_or_directory:path/to/redirection:file|directory program` : pass a redirection rule directly on the terminal
- `boxxy config` : view the config file
- `boxxy -h` : display help

## Categories
files, desktop

## Links
- More information: https://github.com/queer/boxxy
- tldr page: https://tldr.inbrowser.app/pages/common/boxxy
