# tv.fzf

Cross-platform, fast, extensible fuzzy-finder tool. Works with "channels" (e.g., files, env, git repos) or from `stdin`.

## Usage
- `tv` : launch with the default channel
- `tv files|env|git-repos|...` : open a specific channel
- `tv list-channels` : list all available channels
- `command | tv` : fuzzy-pick from lines piped on `stdin`
- `command | tv --preview 'preview_command'` : pipe and show a live preview for each entry
- `tv --source-command 'source_command' --preview-command 'preview_command' --preview-size 70` : build a picker from commands (source + preview)
- `tv update-channels` : update and install community-maintained channels
- `tv tldr` : view tldr pages (you need to update channels first and have `tldr` installed)

## Categories
version-control

## Links
- More information: https://alexpasmantier.github.io/television/
- tldr page: https://tldr.inbrowser.app/pages/common/tv.fzf
