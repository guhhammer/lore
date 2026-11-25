# lore's interface in other languages

lore's screen (the key bar, the about screen, the headings and hints of a
page, the topics, the notes) and the recipes' names, in the language chosen
in **Pages in** on the `+` screen (`system` follows `LANG`). A language with
no folder here, or a text with no line, stays in English.

What lore prints for scripts and agents stays in English on every machine:
`lore -p`, `lore search`, `lore explain`, `--help` and the error messages.

## Files

One folder per language, named like the translations of the pages
(`pt_BR`, `de`; `zh` also serves `zh_CN` and `zh_TW`). `build.rs` builds
every folder in, so a new language is a new folder: no list to update.

- `interface.tsv`: the screen's texts.
- `recipes.tsv`: the recipes' names, as the list and their pages show
  them. The English name stays the recipe's name everywhere else (bookmarks,
  `lore -p "compress a folder"`); the search finds a recipe by either.

Each line is the English text, a tab, then the translation:

```text
VERSION	VERSÃO
{newest}: an update is available	{newest}: há uma atualização
```

- The English text is the key, exactly as lore has it, spaces at either end
  included (` search ` is a box's title).
- `\n` is a new line, `\t` a tab and `\\` a backslash.
- Keep every `{placeholder}` of the English text: lore fills them in.
  The build stops on a line without its tab or with a placeholder missing.
- A line starting with `#` is a comment; blank lines are skipped.
- Key names (`Enter`, `Esc`, `Tab`, `↑↓`) and commands in backquotes stay
  as they are.

`tests/i18n.rs` checks that every language has the same texts and every
recipe, and that every English text is still one of lore's: when a text of
`src/configs/` changes, change its line in every file.

## Languages

ar, de, es, fr, hi, it, ja, ko, pt_BR, pt_PT, ru, zh.
