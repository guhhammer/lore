# gdk-pixbuf-csource

C code generation utility for GdkPixbuf images.

## Usage
- `gdk-pixbuf-csource [OPTIONS...] <IMAGE>` : the general form, as the manual gives it
- `gdk-pixbuf-csource --rle` : enables run-length encoding for the generated pixel data (default)
- `gdk-pixbuf-csource --raw` : disables run-length encoding for the generated pixel data
- `gdk-pixbuf-csource --extern` : generate extern symbols
- `gdk-pixbuf-csource --static` : generate static symbols (default)

## Categories
containers, media

## Links
- Documentation: https://docs.gtk.org/gdk-pixbuf/
