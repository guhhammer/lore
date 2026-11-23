# cat

הדפסה ושרשור של קבצים.

## Usage
- `cat path/to/file` : הדפסת תוכן של קובץ ל-`stdout`
- `cat path/to/file1 path/to/file2 ... >path/to/output_file` : שרשור מספר קבצים לקובץ פלט
- `cat path/to/file1 path/to/file2 ... >>path/to/output_file` : הוספת מספר קבצים לסוף קובץ פלט
- `cat -u /dev/tty12 >/dev/tty13` : העתקת תוכן של קובץ לקובץ פלט ללא חציצה (buffering)
- `cat - >path/to/file` : כתיבה מ-`stdin` לקובץ

## Links
- More information: https://manned.org/cat.1posix
