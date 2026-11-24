# git config

Manage custom configuration options for Git repositories. These configurations can be local (for the current repository) or global (for the current user).

## Usage
- `git config --global user.name|user.email "Your Name|email@example.com"` : globally set your name or email (this information is required to commit to a repository and will be included in all commits)
- `git config --local|global|system -l --show-origin` : list local, global, or system configuration entries and show their file location
- `git config --global alias.unstage "reset HEAD --"` : set the global value of a given configuration entry (in this case an alias)
- `git config alias.unstage` : get the value of a given configuration entry
- `git unstage` : use an alias
- `git config --global --unset alias.unstage` : revert a global configuration entry to its default value
- `git config -e` : edit the local Git configuration (`.git/config`) in the default editor
- `git config --global -e` : edit the global Git configuration (`~/.gitconfig` by default or `$XDG_CONFIG_HOME/git/config` if such a file exists) in the default editor

## Categories
version-control, shell, users

## Links
- More information: https://git-scm.com/docs/git-config
- tldr page: https://tldr.inbrowser.app/pages/common/git-config
- Wikipedia: https://en.wikipedia.org/wiki/Git
