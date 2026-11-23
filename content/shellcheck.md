# shellcheck

Statically check shell scripts for errors, usage of deprecated/insecure features, and bad practices.

## Usage
- `shellcheck path/to/script.sh` : check a shell script
- `shellcheck -s sh|bash|dash|ksh path/to/script.sh` : check a shell script interpreting it as the specified shell dialect (overrides the shebang at the top of the script)
- `shellcheck -e SC1009,SC1073,... path/to/script.sh` : ignore one or more error types
- `shellcheck -a path/to/script.sh` : also check any sourced shell scripts
- `shellcheck -f tty|checkstyle|diff|gcc|json|json1|quiet path/to/script.sh` : display output in the specified format (defaults to `tty`)
- `shellcheck -o add-default-case,avoid-nullary-conditions,... path/to/script.sh` : enable one or more optional checks
- `shellcheck --list-optional` : list all available optional checks that are disabled by default
- `shellcheck -S error|warning|info|style path/to/script.sh` : adjust the level of severity to consider (defaults to `style`)

## Categories
development, shell, files

## Links
- More information: https://github.com/koalaman/shellcheck/wiki
- tldr page: https://tldr.inbrowser.app/pages/common/shellcheck
