# eselect repository

Een `eselect`-module voor het configureren van ebuild-repositories voor Portage. Na het inschakelen van een repository moet je `emerge --sync repo_name` uitvoeren om ebuilds te downloaden.

## Usage
- `eselect repository list` : toon alle ebuild-repositories geregistreerd op <https://repos.gentoo.org>
- `eselect repository list -i` : toon ingeschakelde repositories
- `eselect repository enable naam|index` : schakel een repository uit de lijst in op naam of index van de `list`-opdracht
- `eselect repository add naam rsync|git|mercurial|svn|... sync_uri` : schakel een niet-geregistreerde repository in
- `eselect repository disable repo1 repo2 ...` : schakel repositories uit zonder hun inhoud te verwijderen
- `eselect repository remove repo1 repo2 ...` : schakel repositories uit en verwijder hun inhoud
- `eselect repository create naam pad/naar/repo` : maak een lokale repository aan en schakel deze in

## Links
- More information: https://wiki.gentoo.org/wiki/Eselect/Repository
