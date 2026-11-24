# Palettes

lore's colour palettes, one small file each. Pick one in lore with `+`,
then **Colours**: ↑↓ try them live, Enter keeps one.

| Built in | |
| --- | --- |
| `terminal` | The default: your terminal's own colours. |
| Dark | `catppuccin-mocha`, `dracula`, `gruvbox-dark`, `nord`, `one-dark`, `rose-pine`, `solarized-dark`, `tokyo-night` |
| Light | `catppuccin-latte`, `gruvbox-light`, `one-light`, `rose-pine-dawn`, `solarized-light`, `tokyo-night-day` |

Every `*.palette` here is built into lore, and `build.rs` checks it: a
broken one stops the build with the file and the key named. A palette's
name is its file name: `nord.palette` is `nord`.

---

## Add a palette

1. Copy a palette close to yours.
2. Rename it: lower case, words joined by `-`, a light variant ending in
   its own word (`tokyo-night-day`).
3. Set its colours, and say in the first line where they come from.
4. Run `cli/make`, then open a pull request.

**Just for yourself?** Put the file in `~/.config/lore/palettes/` (or
`$XDG_CONFIG_HOME/lore/palettes/`). lore reads it at start, and one with a
built-in palette's name takes its place.

---

## The format

One `key = value` a line; a line starting with `#` is a comment. Every key
is required:

```text
# nord, from https://www.nordtheme.com/docs/colors-and-palettes

background = #2e3440
text       = #d8dee9
muted      = #616e88
selection  = #434c5e
primary    = #88c0d0
secondary  = #81a1c1
accent     = #8fbcbb
danger     = #bf616a
code       = #a3be8c
warning    = #ebcb8b
```

| Key | What it colours |
| --- | --- |
| `background` | The whole screen. |
| `text` | Plain text. |
| `muted` | Hints, what an alias runs, the dot on never-run commands. |
| `selection` | The selected row's background. |
| `primary` | The theme's main colour: letter headers, page headings, the border of the search bar while typing, the picker. |
| `secondary` | Its second colour: the categories. |
| `accent` | Links and search snippets. |
| `danger` | The `danger` category and risky examples. |
| `code` | Usage examples. |
| `warning` | Warnings, such as the note under the links. |

**Colours** are `#rrggbb`, or one of the terminal's own: `black`, `red`,
`green`, `yellow`, `blue`, `magenta`, `cyan`, `gray`, `darkgray`,
`lightred`, `lightgreen`, `lightyellow`, `lightblue`, `lightmagenta`,
`lightcyan`, `white`, and `reset` (the terminal's default; for `muted` it
dims the text, for `selection` it reverses the row).

**Two rules** (the tests check the first):

- Keep `secondary` different from `primary`, `accent` and `danger`.
- Set `background` and `text` (only `terminal` leaves them to the
  terminal), so a light palette looks the same on a dark terminal.
