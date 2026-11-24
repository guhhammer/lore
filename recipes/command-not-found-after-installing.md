# command not found after installing

You installed a program but the shell says "command not found": find where it went and let the shell see it.

## Steps
- `hash -r` : make the shell forget its old list of commands
- `ls ~/.local/bin ~/.cargo/bin ~/go/bin` : where pip, cargo and go put programs
- `echo "$PATH" | tr : '\n'` : the folders the shell searches
- `exec "$SHELL" -l` : start the shell again, reading the new settings
- `eval "$(/opt/homebrew/bin/brew shellenv)"` : macOS, Homebrew on Apple Silicon: add brew to this shell
- `npm prefix -g` : where npm installs global programs

## Notes
- When the folder is missing from PATH, see "add a folder to the path".

## See also
- hash
- echo
- brew
- npm

## Categories
shell packages
