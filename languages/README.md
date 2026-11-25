# Languages

The pages of [Learn X in Y minutes](https://learnxinyminutes.com): for each
programming language (and some tools and formats), a short introduction
and one well-commented file that walks through how the language works.
They are a reminder of a language's features, read from top to bottom.

In lore they are the **languages** topic (← → in the list). A page opens
like a file in `bat`: the introduction, then the file with line numbers
and colours. Tab and Shift+Tab jump between its parts. In your language,
when there is a translation.

## Where they come from

Copied as they are from
[adambard/learnxinyminutes-docs](https://github.com/adambard/learnxinyminutes-docs),
commit `78548dd` (2026-09-21): 201 pages, and 678 translations in 37
languages under `i18n/<language>/` (the site's `pt-br/` is `pt_BR` here,
the code lore reads from `LANG`). To take newer ones:

```bash
git clone --depth 1 https://github.com/adambard/learnxinyminutes-docs.git .content-draft/lxy
python3 scripts/languages/sync.py .content-draft/lxy
```

Then update the commit above. Corrections belong upstream, on the site's
repository, so every reader gets them.

## Licence

These pages are © their contributors (named in each file's header) and
licensed under
[Creative Commons Attribution-ShareAlike 3.0](https://creativecommons.org/licenses/by-sa/3.0/)
([LICENSE.txt](LICENSE.txt)), not under lore's own licence. They are
unchanged; lore only shows them.
