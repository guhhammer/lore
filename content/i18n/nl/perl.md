# perl

De Perl 5-interpreter.

## Usage
- `perl -n -e 'print if m/regex1/ and m/regex2/i'` : toon regels van `stdin` die overeenkomen met `regex1` en hoofdletterongevoelig `regex2`
- `perl -n -E 'say $1 if m/voor ( groep_regex ) na/x'` : GEef de eerste overeenkomende groep op, met behulp van `regex`, waarbij de spatie in `regex` wordt genegeerd (x)
- `perl -i'.bak' -p -e 's/regex/vervanging/g' pad/naar/bestanden` : ter plaatse (i), met backup, vervang (s) alle voorkomens (g) van `regex` met een vervanging

## Links
- More information: https://perldoc.perl.org/perl
