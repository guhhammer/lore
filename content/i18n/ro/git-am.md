# git am

Aplică fișiere patch și creează un commit. Util când se primesc commit-uri prin email.

## Usage
- `git am cale/către/fișier.patch` : aplică și confirmă modificările urmând un fișier patch local
- `curl -L https://example.com/file.patch | git am` : aplică și confirmă modificările urmând un fișier patch la distanță
- `git am --abort` : întrerupe procesul de aplicare a unui fișier patch
- `git am --reject cale/către/fișier.patch` : aplică cât este posibil dintr-un fișier patch, salvând fragmentele eșuate în fișiere de respingere

## Links
- More information: https://git-scm.com/docs/git-am
