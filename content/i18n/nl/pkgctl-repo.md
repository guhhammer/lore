# pkgctl repo

Beheer Git verpakkingsrepositories en hun configuratie voor Arch Linux.

## Usage
- `pkgctl repo clone pkgname` : kloon een pakketrepository (vereist het instellen van een SSH-key in uw Arch Linux GitLab-account)
- `pkgctl repo clone --protocol https pkgname` : kloon een pakketrepository via HTTPS
- `pkgctl repo create pkgbase` : maak een nieuwe GitLab-pakketrepository en kloon deze na het aanmaken (vereist geldige GitLab-API authenticatie)
- `pkgctl repo switch versie pkgbase` : wissel een pakketrepository naar een specifieke versie
- `pkgctl repo web pkgbase` : open de website van een pakketrepository

## Links
- More information: https://manned.org/pkgctl-repo
