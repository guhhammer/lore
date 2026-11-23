# git mergetool

Run merge conflict resolution tools to resolve merge conflicts.

## Usage
- `git mergetool` : launch the default merge tool to resolve conflicts
- `git mergetool --tool-help` : list valid merge tools
- `git mergetool -t tool_name` : launch the merge tool identified by a name
- `git mergetool -y` : don't prompt before each invocation of the merge tool
- `git mergetool -g` : explicitly use the GUI merge tool (see the `merge.guitool` configuration variable)
- `git mergetool --no-gui` : explicitly use the regular merge tool (see the `merge.tool` configuration variable)

## Categories
version-control

## Links
- More information: https://git-scm.com/docs/git-mergetool
- tldr page: https://tldr.inbrowser.app/pages/common/git-mergetool
