# urpmf

Find files in packages and query information about them in Mageia.

## Usage
- `urpmf filename` : search for packages that contain a file
- `urpmf --summary keyword1 -a keyword2` : search for packages that contain both a keyword and another in their summaries
- `urpmf --description keyword1 -o keyword2` : search for packages that contain a keyword or another in their descriptions
- `urpmf --description ! keyword -F'|'` : search for packages that do not contain a keyword in their name ignoring case distinction using "|" as the Field separator (":" by default)

## See also
- urpmi
- urpme
- urpmi.addmedia
- urpmi.removemedia
- urpmi.update
- urpmq

## Categories
packages, databases

## Links
- More information: https://man.linuxreviews.org/man8/urpmf.8.html
- tldr page: https://tldr.inbrowser.app/pages/linux/urpmf
