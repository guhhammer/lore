# b4 am

Retrieve and prepare patches from a public-inbox archive for applying with `git am`.

## Usage
- `b4 am message_id` : retrieve patches from a thread and prepare them for `git am`
- `b4 am -sl message_id` : retrieve patches and add your own `Signed-off-by` trailer as well as a `Link:` trailer to every commit
- `b4 am -3 message_id` : retrieve patches and prepare with 3-way merge support for conflict resolution
- `b4 am -P 1-3,5 message_id` : cherry-pick a subset of patches (e.g., patches 1-3 and 5) from a series
- `b4 am --check message_id` : retrieve patches and run local checks on each patch
- `b4 am -o path/to/directory message_id` : save the output mailbox to a specific directory

## Categories
version-control, files, archives

## Links
- More information: https://b4.docs.kernel.org/en/latest/maintainer/am-shazam.html
- tldr page: https://tldr.inbrowser.app/pages/linux/b4-am
