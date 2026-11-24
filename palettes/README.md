# Palettes

lore's colour palettes, one file each. Every `*.palette` here is built
into lore (`build.rs` checks it, so a broken one fails the build with the
file and the key named), and shows in the picker (`+`, then Colours) by its file name:
`nord.palette` is `nord`. `terminal.palette` is the default and comes first;
the others follow by name.

## Adding one

Copy a palette close to yours, rename it (lower case, words joined by `-`,
a light variant ending in its own word: `tokyo-night-day`), set its
colours, run `cli/make`, and open a pull request. Say in the first line
where the colours come from.

The same file also works without a pull request: put it in
`~/.config/lore/palettes/` (`$XDG_CONFIG_HOME/lore/palettes/`) and lore
reads it at start; one with a built-in palette's name takes its place.

## The format

One `key = value` a line; `#` at the start of a line is a comment. Every
key is needed:

| Key | What it colours |
| --- | --- |
| `background` | the whole screen |
| `text` | plain text |
| `muted` | hints, what an alias runs, the dot on never-run commands |
| `selection` | the selected row's background |
| `primary` | the theme's main colour: letter headers, page headings, the border of the search bar while typing and of the picker |
| `secondary` | its second colour: the categories |
| `accent` | links and search snippets |
| `danger` | the `danger` category and risky examples |
| `code` | usage examples |
| `warning` | warnings, like the note under the links |

A colour is `#rrggbb`, or one of the terminal's own: `black`, `red`,
`green`, `yellow`, `blue`, `magenta`, `cyan`, `gray`, `darkgray`,
`lightred`, `lightgreen`, `lightyellow`, `lightblue`, `lightmagenta`,
`lightcyan`, `white`, and `reset` (the terminal's default; for `muted` it
dims the text, for `selection` it reverses the row).

Keep `secondary` apart from `primary`, `accent` and `danger` (a test checks
it), and paint `background` and `text` (only `terminal` leaves them to the
terminal), so a light palette reads the same on a dark terminal.

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
