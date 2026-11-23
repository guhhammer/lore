# gource

Render an animated tree diagram of Git, SVN, Mercurial, and Bazaar repositories. It shows files and directories being created, modified, or removed over time.

## Usage
- `gource path/to/repository` : run gource in a directory (if it isn't the repository's root directory, the root is sought up from there)
- `gource -widthxheight` : run gource in the current directory, with a custom output resolution
- `gource -s seconds` : specify how long each day should be in the animation (this combines with -c, if provided)
- `gource -f -b hex_color_code` : use fullscreen mode and a custom background color
- `gource --follow-user user_name` : highlight the name of a specific user
- `gource --title title` : specify the animation title
- `<Space>` : pause the animation
- `<+|-|<|>>` : adjust simulation speed and timescale

## Categories
version-control, files

## Links
- More information: https://manned.org/gource
- tldr page: https://tldr.inbrowser.app/pages/common/gource
