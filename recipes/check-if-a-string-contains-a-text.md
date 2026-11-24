# check if a string contains a text

Test in a script whether a string contains, starts with or matches something.

## Steps
- `[[ $text == *"needle"* ]] && echo found` : contains needle
- `[[ $text == prefix* ]] && echo yes` : starts with prefix
- `[[ $text =~ ^[0-9]+$ ]] && echo 'a number'` : matches a regular expression
- `grep -q 'needle' <<< "$text" && echo found` : with grep, also in sh
- `[[ -z $text ]] && echo empty` : is empty

## Notes
- [[ ]] is bash and zsh; in plain sh use case or grep.

## See also
- grep

## Categories
shell development
