# eselect repository

An `eselect` module for configuring ebuild repositories for Portage. After enabling a repository, you have to run `emerge --sync repo_name` to download ebuilds.

## Usage
- `eselect repository list` : list all ebuild repositories registered on <https://repos.gentoo.org>
- `eselect repository list -i` : list enabled repositories
- `eselect repository enable name|index` : enable a repository from the list by its name or index from the `list` command
- `eselect repository add name rsync|git|mercurial|svn|... sync_uri` : enable an unregistered repository
- `eselect repository disable repo1 repo2 ...` : disable repositories without removing their contents
- `eselect repository remove repo1 repo2 ...` : disable repositories and remove their contents
- `eselect repository create name path/to/repo` : create a local repository and enable it

## Categories
packages, network, development

## Links
- More information: https://wiki.gentoo.org/wiki/Eselect/Repository
- tldr page: https://tldr.inbrowser.app/pages/linux/eselect-repository
