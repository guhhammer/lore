# h2ph

Convert .h C header files to .ph Perl header files.

## Usage
- `h2ph [-d destination directory] [-r | -a] [-l] [-h] [-e] [-D] [-Q] [headerfiles]` : the general form, as the manual gives it
- `h2ph -l` : symbolic links will be replicated in the destination directory
- `h2ph -D` : include the code from the .h file as a comment in the .ph file
- `h2ph -Q` : 'Quiet' mode; don't print out the names of the files being converted

## Categories
files
