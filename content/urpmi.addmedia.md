# urpmi.addmedia

Add media in Mageia. Note: Mageia documentation uses medium and repository as synonymous.

## Usage
- `sudo urpmi.addmedia medium ftp://ftp.site.com/path/to/Mageia/RPMS` : add a medium
- `sudo urpmi.addmedia --distrib HD file://path/to/repo` : add a medium from a hard drive (run `genhdlist2` in the directory first)
- `sudo urpmi.addmedia --distrib ftp://mirror_website/mirror/mageia/distrib/version/arch` : add important media from a chosen mirror
- `sudo urpmi.addmedia --distrib --mirrorlist mirrorlist` : automatically select mirrors from a mirror list

## See also
- urpmi
- urpmi.update
- urpme
- urpmi.removemedia
- urpmf
- urpmq

## Categories
network, files, users

## Links
- More information: https://man.linuxreviews.org/man8/urpmi.addmedia.8.html
- tldr page: https://tldr.inbrowser.app/pages/linux/urpmi.addmedia
