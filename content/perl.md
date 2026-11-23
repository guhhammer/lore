# perl

The Perl 5 language interpreter.

## Usage
- `perl -n -e 'print if m/regex1/ and m/regex2/i'` : print lines from `stdin` matching `regex1` and case insensitive `regex2`
- `perl -n -E 'say $1 if m/before ( group_regex ) after/x'` : say ([-E]) first match group, using a `regex`, ignoring space (x) in `regex`
- `perl -i'.bak' -p -e 's/regex/replacement/g' path/to/files` : in-place, with backup, substitute all occurrence (g) of `regex` with a replacement

## See also
- perldoc

## Categories
text, development

## Links
- More information: https://perldoc.perl.org/perl
- tldr page: https://tldr.inbrowser.app/pages/common/perl
