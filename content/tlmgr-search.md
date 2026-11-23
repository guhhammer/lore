# tlmgr search

Search for TeX Live packages using (Perl) `regex`.

## Usage
- `tlmgr search "regex"` : search for a package name and descriptions of all locally installed packages from a specific `regex`
- `tlmgr search --file "regex"` : search for all file names of all locally installed packages from a `regex`
- `tlmgr search --all "regex"` : search for all file names, package names, and descriptions of all locally installed packages from a `regex`
- `tlmgr search --global "regex"` : search the TeX Live database, instead of the local installation
- `tlmgr search --all --word "regex"` : restrict the matches for package names and descriptions (but not for file names) to whole words

## Categories
text, packages, files

## Links
- More information: https://www.tug.org/texlive/doc/tlmgr.html#search
- tldr page: https://tldr.inbrowser.app/pages/common/tlmgr-search
