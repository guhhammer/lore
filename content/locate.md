# locate

Find filenames quickly.

## Usage
- `locate "pattern"` : look for pattern in the database. Note: The database is recomputed periodically (usually weekly or daily)
- `locate -i "pattern"` : look for a pattern case-insensitively
- `locate "*/filename"` : look for a file by its exact filename (a pattern containing no globbing characters is interpreted as `*pattern*`)
- `sudo updatedb` : recompute the database. You need to do it if you want to find recently added files

## Categories
files

## Links
- More information: https://manned.org/locate
- tldr page: https://tldr.inbrowser.app/pages/linux/locate
- Wikipedia: https://en.wikipedia.org/wiki/Locate_(Unix)
