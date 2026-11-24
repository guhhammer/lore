# git mailinfo

Extract patch and authorship information from a single email message.

## Usage
- `git mailinfo message|patch` : extract the patch and author data from an email message
- `git mailinfo -k message|patch` : extract but remove leading and trailing whitespace
- `git mailinfo --scissors message|patch` : remove everything from the body before a scissors line (e.g. "-->* --") and retrieve the message or patch

## Categories
version-control, archives

## Links
- More information: https://git-scm.com/docs/git-mailinfo
- tldr page: https://tldr.inbrowser.app/pages/common/git-mailinfo
- Wikipedia: https://en.wikipedia.org/wiki/Git
