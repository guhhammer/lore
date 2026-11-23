# gnumfmt

This command is an alias of GNU `numfmt`.

## Usage
- `numfmt --from si 1.5K` : convert 1.5K (SI Units) to 1500
- `numfmt --to si 1500` : convert 1500 to 1.5K (SI Units)
- `numfmt --from iec 1.5K` : convert 1.5K (IEC Units) to 1536
- `numfmt --from auto 1.5Ki` : use appropriate conversion based on the suffix
- `ls -l | numfmt --header=1 --field 5 --to iec` : convert 5th field (1-indexed) to IEC Units without converting header
- `du -s * | numfmt --to iec --format "%-5f"` : convert to IEC units, pad with 5 characters, left aligned

## Categories
shell, macos

## Links
- tldr page: https://tldr.inbrowser.app/pages/osx/gnumfmt
