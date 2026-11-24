# argument list too long

"Argument list too long" from rm, cp or mv: too many files for one command line. Hand them over in batches.

## Steps
- `find . -maxdepth 1 -name '*.log' -delete` : delete them without listing them on the command line
- `find . -maxdepth 1 -name '*.jpg' -exec mv -t /dest/ {} +` : move them, many at a time (GNU mv)
- `printf '%s\0' *.log | xargs -0 rm` : xargs makes batches that fit
- `getconf ARG_MAX` : how long a command line can be here

## See also
- find
- xargs
- getconf

## Categories
files shell
