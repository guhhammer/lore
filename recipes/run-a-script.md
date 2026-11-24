# run a script

Make a script runnable and run it, or run it in the shell you are in.

## Steps
- `chmod +x script.sh` : let it be run
- `./script.sh` : run it from its folder
- `bash script.sh` : run it with bash, even without the x
- `head -1 script.sh` : its first line, #!/usr/bin/env bash, says what runs it
- `source script.sh` : run it in this shell, so its cd and variables stay

## Notes
- "bad interpreter" with ^M: see "fix windows line endings".

## See also
- chmod
- bash
- source

## Categories
shell
