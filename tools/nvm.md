# nvm

nvm installs several versions of Node.js side by side and switches between them per terminal or per project. It is a shell function, not a program in your PATH.

## Tasks
### Install versions
- `nvm install --lts` : install the newest long-term-support Node
- `nvm install 22` : install the latest 22.x
- `nvm install 20.11.1` : install an exact version
- `nvm install node` : install the very latest release
- `nvm install` : install the version named in .nvmrc
### Switch
- `nvm use 22` : use a version in this terminal
- `nvm use` : use the version in .nvmrc
- `nvm use --lts` : use the newest LTS
- `nvm alias default 22` : the version every new terminal starts with
- `nvm run 20 app.js` : run a script with one version
- `nvm exec 20 node -v` : run any command with one version
### Look
- `nvm ls` : versions installed here
- `nvm ls-remote --lts` : LTS versions you could install
- `nvm current` : the version in use now
- `nvm which 22` : the path of a version's node
- `node -v` : check the version node reports
### Clean up
- `nvm uninstall 18` : remove a version (deletes it)
- `nvm cache clear` : empty the download cache
- `nvm install 22 --reinstall-packages-from=20` : carry global packages over to a new version
### Per project
- `node -v > .nvmrc` : pin the project's Node version in a file
- `echo 22 > .nvmrc` : the same, by hand
- `nvm version-remote --lts` : show the newest LTS number
- `nvm deactivate` : undo nvm's changes to PATH in this terminal

## Notes
- nvm is loaded by your shell startup file (`~/.bashrc`, `~/.zshrc`): open a new terminal after installing it.
- Global npm packages belong to one Node version; install them again after a switch.
- It does not support fish natively and does not run on Windows: see nvm-windows or fnm.
- Get the installer from the official page in Links and read it before you run it.

## See also
- npm
- pnpm
- node
- fnm

## Links
- Project and install: https://github.com/nvm-sh/nvm

## Categories
development
