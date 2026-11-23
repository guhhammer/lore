# conda init

Initialize conda for shell interaction. Most shells need to be closed and restarted for changes to take effect.

## Usage
- `conda init zsh|bash|powershell|fish|tcsh|xonsh` : initialize a specific shell (if none is specified, defaults to `bash` for UNIX and `powershell` for Windows)
- `conda init --all` : initialize all available shells
- `conda init --system` : initialize conda for all users on the system
- `conda init --no-user` : don't initialize conda for the current user
- `conda init --condabin` : add `condabin/` directory to `$PATH`
- `conda init --reverse` : undo effects of the last `conda init`

## Categories
shell, users

## Links
- More information: https://docs.conda.io/projects/conda/en/latest/commands/init.html
- tldr page: https://tldr.inbrowser.app/pages/common/conda-init
