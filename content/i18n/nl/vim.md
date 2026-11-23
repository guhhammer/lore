# vim

Vim (Vi IMproved), een command-line tekstbewerker, geeft toegang tot verschillende manieren van tekstmanipulatie. Drukken op `<i>` in de normale modus begint de invoegmodus. `<Esc>` gaat terug naar de normale modus, wat toegang geeft tot de Vim-commando's.

## Usage
- `vim pad/naar/bestand` : open een bestand
- `vim +regelnummer pad/naar/bestand` : open een bestand bij een bepaald regelnummer
- `<:>help<Enter>` : bekijk de handleiding van Vim
- `<Esc><Z><Z>|<Esc><:>x<Enter>|<Esc><:>wq<Enter>` : sla de huidige buffer op en sluit af
- `<Esc><u>` : ga terug naar de normale modus en maak de laatste verandering ongedaan
- `</>zoek_patroon<Enter>` : zoek een patroon in het bestand (druk op `<n>`/`<N>` om naar de volgende/vorige overeenkomst te gaan)
- `<:>%s/regex/vervanging/g<Enter>` : voer een `regex`-substitutie uit in het hele bestand
- `<:>set nu<Enter>` : geef de regelnummers weer

## Links
- More information: https://www.vim.org/
