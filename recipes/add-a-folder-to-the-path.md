# add a folder to the path

Let the shell find programs in a folder, for good, and reload the settings without logging out.

## Steps
- `echo "$PATH" | tr : '\n'` : the folders searched now, one a line
- `export PATH="$HOME/bin:$PATH"` : add ~/bin for this shell
- `echo 'export PATH="$HOME/bin:$PATH"' >> ~/.bashrc` : add it for good (bash)
- `source ~/.bashrc` : load the change now, without logging out
- `fish_add_path ~/bin` : fish: adds it for good

## Notes
- zsh: ~/.zshrc. Login shells on macOS read ~/.zprofile or ~/.bash_profile.

## See also
- export
- source

## Categories
shell
