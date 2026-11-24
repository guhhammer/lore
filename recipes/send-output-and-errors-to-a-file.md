# send output and errors to a file

Keep what a command prints, its errors too, in a file, or throw them away.

## Steps
- `command > out.log 2>&1` : the output and the errors into out.log (2>&1: errors go where the output goes)
- `command >> out.log 2>&1` : the same, added at the end of the file
- `command 2> errors.log` : only the errors into a file
- `command 2>/dev/null` : throw the errors away
- `command > /dev/null 2>&1` : throw everything away
- `command 2>&1 | tee out.log` : see everything and keep a copy

## Notes
- The order matters: `> out.log 2>&1` works, `2>&1 > out.log` sends the errors to the screen.
- `lore explain "command > out.log 2>&1"` says what each part does.

## See also
- tee

## Categories
shell
