# portageq

Query for information about Portage, the Gentoo Linux package manager. Queryable Portage-specific environment variables are listed in `/var/db/repos/gentoo/profiles/info_vars`.

## Usage
- `portageq envvar variable` : display the value of a Portage-specific environment variable
- `portageq repos_config /` : display a detailed list of repositories configured with Portage
- `portageq get_repos /` : display a list of repositories sorted by priority (highest first)
- `portageq metadata / ebuild|porttree|binary|... category/package BDEPEND|DEFINED_PHASES|DEPEND|...` : display a specific piece of metadata about an atom (i.e. package name including the version)

## Categories
packages, desktop, databases

## Links
- More information: https://wiki.gentoo.org/wiki/Portageq
- tldr page: https://tldr.inbrowser.app/pages/linux/portageq
