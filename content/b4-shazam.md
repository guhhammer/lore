# b4 shazam

Retrieve and apply patches from a public-inbox archive directly to the current Git tree.

## Usage
- `b4 shazam message_id` : retrieve patches from a thread and apply them directly to the current branch
- `b4 shazam -H message_id` : retrieve patches and create a `FETCH_HEAD` for merging (like a pull request)
- `b4 shazam -M message_id` : retrieve patches and automatically merge using the cover letter as the merge commit message
- `b4 shazam -slM message_id` : retrieve patches, add your `Signed-off-by` and `Link:` trailers, then merge
- `b4 shazam -H --resolve message_id` : retrieve patches and start interactive conflict resolution when applicable

## Categories
version-control, archives

## Links
- More information: https://b4.docs.kernel.org/en/latest/maintainer/am-shazam.html
- tldr page: https://tldr.inbrowser.app/pages/linux/b4-shazam
