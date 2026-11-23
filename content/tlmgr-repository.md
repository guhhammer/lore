# tlmgr repository

Manage repositories of a TeX Live installation.

## Usage
- `tlmgr repository list` : list all configured repositories and their tags (if set)
- `tlmgr repository list path|url|tag` : list all packages available in a specific repository
- `sudo tlmgr repository add path|url tag` : add a new repository with a specific tag (the tag is not required)
- `sudo tlmgr repository remove path|url|tag` : remove a specific repository
- `sudo tlmgr repository set path|url|tag#tag path|url|tag#tag ...` : set a new list of repositories, overwriting the previous list
- `tlmgr repository status` : show the verification status of all configured repositories

## Categories
packages, network, files

## Links
- More information: https://www.tug.org/texlive/doc/tlmgr.html#repository
- tldr page: https://tldr.inbrowser.app/pages/common/tlmgr-repository
